use actix_web::web;
use crate::handlers::add_handlers::create_book;
pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(create_book);
}