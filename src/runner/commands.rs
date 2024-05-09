use super::models::SupportedLangs;

pub fn get_command(lang: SupportedLangs, id: &str, path: &str) -> (String, Vec<String>) {
    let code_path = format!("{path}:/code");
    let mut common_args = vec!["run", "--rm", "-m", "64M", "--memory-swap", "64M", "--name", id, "-v", &code_path, "-w", "/code"];
    
    let rust_args = vec!["cpp", "/bin/sh", "-c", "\"g++ -Wall main.cpp -o a && ./a >&1 | tee\""];
    let javascript_args = vec!["node:current-alpine3.15", "/bin/sh", "-c", "node main.js"];
    let python_args = vec!["python:alpine", "/bin/sh", "-c", "\"python main.py\""];

    let args = match lang {
        SupportedLangs::Rust => rust_args,
        SupportedLangs::Javascript => javascript_args,
        SupportedLangs::Python => python_args,
    };

    common_args.extend_from_slice(&args);

    ("docker".to_string(), common_args.into_iter().map(|arg| arg.to_string()).collect())
}