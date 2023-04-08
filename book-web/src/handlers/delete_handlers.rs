use actix_web::{delete, web,  HttpResponse};
use crate::models::list::*;
use serde_json::json;

#[delete("/books/{id}")]
async fn delete_book(category: web::Json<DeleteBook>,list_id: web::Path<i32>) -> HttpResponse {
    let req = category.into_inner();
    let id: i32 = list_id.to_string().parse().unwrap();
    let message = "Delete Complete".to_string();
    let messageeror = "Id Not Found".to_string();
    let mut _book = DeleteBook{
        category:req.category,
        list:req.list

    };
    if id != 20 {
        let respondbook = DeleteMessage{
            message: messageeror
        };
        let response_body = json!(respondbook);
        HttpResponse::NotFound().json(response_body)

    }else {
        let respondbook = DeleteMessage{
            message: message
        };
        let response_body = json!(respondbook);
        HttpResponse::Ok().json(response_body)  
    }
}