use actix_web::web;
use crate::handlers::list_handlers::get_books;
pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(get_books);
}