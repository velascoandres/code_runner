use actix_web::{post, web, HttpResponse, Responder};

use crate::runner::{
    executer::Executer,
    models::{Submission, SubmissionResponse},
};
use validator::Validate;

#[post("/execute")]
pub async fn execute_code(json: web::Json<Submission>) -> impl Responder {
    let is_valid = json.validate();

    if is_valid.is_err() {
        let error = is_valid.err().unwrap();

        return HttpResponse::BadRequest().json(error);
    }

    let execution_result = Executer::new("test", json.into_inner()).execute();

    match execution_result {
        Ok(output) => {
            HttpResponse::Ok().json(SubmissionResponse { 
                is_success: true, 
                output,
                results: vec![] 
            })
        },
        Err(err) => {
            HttpResponse::BadRequest().json(SubmissionResponse { 
                is_success: false, 
                output: err.to_string(),
                results: vec![] 
            })
        },
    }
}
