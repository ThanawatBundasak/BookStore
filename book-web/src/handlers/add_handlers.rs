use actix_web::{post, web,  HttpResponse};
use crate::models::list::*;
use serde_json::json;

#[post("/books/{id}")]
async fn create_book(category: web::Json<RequestBook>,list_id: web::Path<i32>) -> HttpResponse {
    let req = category.into_inner();
    let id: i32 = list_id.to_string().parse().unwrap();
    let messageeror = "Not Found".to_string();
    let date = "2023-03-19T08:40:51.620Z";
    let book = RequestBook{
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
        let respondbook = BookCategory{
            category: book.category,
            published_at: date.to_string(),
            list: book.list
        };
        let response_body = json!(respondbook);
        HttpResponse::Created().json(response_body)
    
    }
}

