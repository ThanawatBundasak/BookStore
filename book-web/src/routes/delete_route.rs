use actix_web::web;
use crate::handlers::delete_handlers::delete_book;
pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(delete_book);
}