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

    let execution_result = Executer::new(json.into_inner()).execute();

    match execution_result {
        Ok(output) => {
            HttpResponse::Ok().json(SubmissionResponse { 
                is_success: true, 
                message: "Submission was executed".to_string(),
                results: output
            })
        },
        Err(err) => {
            HttpResponse::BadRequest().json(SubmissionResponse { 
                is_success: false, 
                message: err.to_string(),
                results: vec![] 
            })
        },
    }
}
