use actix_web::web;
use crate::handlers::update_handlers::update_book;
pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(update_book);
}