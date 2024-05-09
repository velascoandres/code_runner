use actix_web::web;

use crate::runner;

pub fn setup_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    cfg.service(runner::handlers::execute_code)
}


pub fn initialize(cfg: &mut web::ServiceConfig) {
    setup_routes(cfg);
}