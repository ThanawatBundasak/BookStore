use actix_web::web;
use crate::handlers::profile_handlers::get_id;

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(get_id);
}