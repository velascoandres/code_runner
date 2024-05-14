use actix_web::{post, web, HttpResponse, Responder};

use crate::{
    adapter,
    runner::{
        executer::Executer,
        models::{Submission, SubmissionResponse},
    },
};
use validator::Validate;

#[post("/execute")]
pub async fn execute_code(json: web::Json<Submission>) -> impl Responder {
    let is_valid = json.validate();

    if is_valid.is_err() {
        let error = is_valid.err().unwrap();

        return HttpResponse::BadRequest().json(error);
    }

    let submission = json.into_inner();

    let execution_result = match submission.supported_lang() {
        crate::runner::models::SupportedLangs::Rust => {
            Executer::new(submission).execute(adapter::rust::RustAdapter)
        }
        crate::runner::models::SupportedLangs::Javascript => {
            Executer::new(submission).execute(adapter::javascript::JavascriptAdapter)
        }
        crate::runner::models::SupportedLangs::Python => {
            Executer::new(submission).execute(adapter::python::PythonAdapter)
        }
    };

    match execution_result {
        Ok(output) => HttpResponse::Ok().json(SubmissionResponse {
            is_success: true,
            message: "Submission was executed".to_string(),
            results: output,
        }),
        Err(err) => HttpResponse::BadRequest().json(SubmissionResponse {
            is_success: false,
            message: err.to_string(),
            results: vec![],
        }),
    }
}
