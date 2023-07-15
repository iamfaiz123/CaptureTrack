mod models;
use actix_web::web;
pub fn api_cfg(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/v1"));
}
