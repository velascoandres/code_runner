use crate::adapter::lang_adapter::{CodeInfo, LangAdapter, RunArgs};
use crate::runner::errors::ExecutionError;

use super::models::Submission;
use super::models::{InputResult, SupportedLangs};
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct CodePathInfo {
    pub absolute_path: String,
    pub relative_path: String,
    pub solution_filename: String,
    pub main_filename: String,
    pub extension: String,
}

impl CodePathInfo {
    pub fn new(path_str: &str, lang: SupportedLangs) -> Self {
        let path = Path::new(path_str);

        let binding = std::env::current_dir().unwrap();
        let base_dir = binding.as_path();
        let absolute_path = Path::new(base_dir)
            .join(path)
            .as_path()
            .to_str()
            .unwrap()
            .to_string();
        let relative_path = path.to_str().unwrap().to_string();

        let extension = match lang {
            crate::runner::models::SupportedLangs::Rust => "rs",
            crate::runner::models::SupportedLangs::Javascript => "js",
            crate::runner::models::SupportedLangs::Python => "py",
        };

        Self {
            absolute_path,
            relative_path,
            solution_filename: format!("solution.{extension}"),
            main_filename: format!("main.{extension}"),
            extension: extension.to_string(),
        }
    }
}

pub struct Executer {
    submission: Submission,
}

impl Executer {
    pub fn new(submission: Submission) -> Self {
        Self { submission }
    }

    pub fn path_info(&self) -> CodePathInfo {
        let id = self.submission.id.clone();

        CodePathInfo::new(
            &format!("submissions/{id}"),
            self.submission.supported_lang(),
        )
    }

    pub fn execute<T: LangAdapter>(
        &self,
        lang_adapter: T,
    ) -> Result<Vec<InputResult>, ExecutionError> {
        let path_info = self.path_info();

        lang_adapter.setup_environment(
            &path_info,
            &CodeInfo {
                id: self.submission.id.clone(),
                solution_code: self.submission.solution_code.clone(),
                main_code: self.submission.main_code.clone(),
            },
        )?;

        let mut results: Vec<InputResult> = vec![];

        for input in self.submission.inputs.iter() {
            let build_args = RunArgs {
                id: self.submission.id.clone(),
                path: path_info.absolute_path.clone(),
                params: input.args.clone(),
            };

            let command = lang_adapter.make_run_command(build_args);

            let output_result = Command::new(command.0)
                .args(command.1)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .map(|output| {
                    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();

                    if !result.is_empty() {
                        results.push(InputResult {
                            input: input.args.clone(),
                            output: result,
                            expected_result: input.expected_result.clone(),
                        });
                    } else {
                        results.push(InputResult {
                            input: input.args.clone(),
                            output: String::from_utf8_lossy(&output.stderr).trim().to_string(),
                            expected_result: input.expected_result.clone(),
                        });
                    }
                });

            if let Err(err) = output_result {
                return Err(ExecutionError::ExecutionError(format!(
                    "Error on execute code: {err}"
                )));
            }
        }

        lang_adapter.clean_up(path_info)?;

        Ok(results)
    }
}
