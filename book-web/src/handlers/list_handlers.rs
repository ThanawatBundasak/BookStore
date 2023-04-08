use actix_web::{get, HttpResponse};
use crate::models::list::Book;
use crate::models::list::BookList;
use crate::models::list::BookCategory;
//use serde_json::json;

#[get("/books")]
async fn get_books() -> HttpResponse {
    let book_list = BookList {
        books: vec![
            BookCategory {
                category: String::from("kid"),
                published_at: String::from("2023-03-19T08:40:51.620Z"),
                list: vec![
                    Book {
                        book_id: 1,
                        book_name: String::from("Little Red Riding Hood"),
                        price: 100,
                        stock: 6,
                    },
                    Book {
                        book_id: 2,
                        book_name: String::from("Peterpan"),
                        price: 120,
                        stock: 8,
                    },
                    Book {
                        book_id: 3,
                        book_name: String::from("Alice In Wonderland"),
                        price: 100,
                        stock: 12,
                    },
                    Book {
                        book_id: 4,
                        book_name: String::from("Pinocchio"),
                        price: 120,
                        stock: 15,
                    },
                ],
            },
            BookCategory {
                category: String::from("education"),
                published_at: String::from("2023-03-19T08:50:51.620Z"),
                list: vec![
                    Book {
                        book_id: 5,
                        book_name: String::from("คณิตไม่ต้องคิด"),
                        price: 100,
                        stock: 2,
                    },
                    Book {
                        book_id: 6,
                        book_name: String::from("ไทยวิบัติ"),
                        price: 120,
                        stock: 6,
                    },
                    Book {
                        book_id: 7,
                        book_name: String::from("เคมีแห่งนรก"),
                        price: 100,
                        stock: 3,
                    },
                    Book {
                        book_id: 8,
                        book_name: String::from("แหกกฎหมาย"),
                        price: 120,
                        stock: 1,
                    },
                ],
            },
        ],
    };
    HttpResponse::Ok().json(book_list)
}

