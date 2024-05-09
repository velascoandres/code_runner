use crate::runner::errors::ExecutionError;

use super::models::SupportedLangs;
use super::{commands::get_command, models::Submission};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::{fs::File, io::Write};

#[derive(Debug)]
pub struct CodePathInfo {
    pub absolute_path: String,
    pub relative_path: String,
    pub filename: String,
}


impl CodePathInfo {
    pub fn new(path_str: &str, lang: SupportedLangs) -> Self {
        let path = Path::new(path_str);

        let binding = std::env::current_dir().unwrap();
        let base_dir = binding.as_path();
        let absolute_path = Path::new(base_dir).join(path).as_path().to_str().unwrap().to_string();
        let relative_path = path.to_str().unwrap().to_string();

        let extension = match lang {
            crate::runner::models::SupportedLangs::Rust => "rs",
            crate::runner::models::SupportedLangs::Javascript => "js",
            crate::runner::models::SupportedLangs::Python => "py",
        };

        Self {
            absolute_path,
            relative_path,
            filename: format!("main.{extension}"),
        }
    }
}

pub struct Executer {
    id: String,
    submission: Submission,
}

impl Executer {
    pub fn new(id: &str, submission: Submission) -> Self {
        Self {
            id: String::from(id),
            submission,
        }
    }

    pub fn path_info(&self) -> CodePathInfo {
        let id = self.id.clone();

        CodePathInfo::new(
            &format!("submissions/{id}"),
            self.submission.supported_lang(),
        )
    }

    pub fn execute(&self) -> Result<String, ExecutionError> {
        self.setup_files()?;

        let path_info = self.path_info();
        let command = get_command(
            self.submission.supported_lang(),
            &self.id,
            &path_info.absolute_path,
        );
        
        let output = Command::new(command.0).args(command.1).output();

        match output {
            Ok(result) => Ok(String::from_utf8_lossy(&result.stdout).to_string()),
            Err(err) => Err(ExecutionError::RuntimeError(format!("Error on execute code: {err}")))
        }
    }

    fn setup_files(&self) -> Result<(), ExecutionError> {
        let path_info = self.path_info();

        println!("{:?}", path_info);

        let relative_path = path_info.relative_path;
        let filename = path_info.filename;
        let file_path = format!("{relative_path}/{filename}");
        let complete_path = Path::new(&file_path);

        if complete_path.parent().is_none() {
            return  Err(ExecutionError::ExecutionEnvironmentError("Path empty".to_string()));
        }

        let parent_path = complete_path.parent().unwrap();

        if let Err(err) = fs::create_dir_all(parent_path) {
            return Err(ExecutionError::ExecutionEnvironmentError(format!("Error creating folder: {err}")))
        }

        match File::create(complete_path).and_then(|mut file| file.write_all(self.submission.code.as_bytes())) {
            Ok(_) => {
                Ok(())
            },
            Err(err) => {
                Err(ExecutionError::ExecutionEnvironmentError(format!("Error creating directory: {err}")))
            },
        } 
    }
}
