use strum_macros::Display;

#[derive(Display)]
pub enum ExecutionError {
    RuntimeError(String),
    ExecutionEnvironmentError(String)
}
