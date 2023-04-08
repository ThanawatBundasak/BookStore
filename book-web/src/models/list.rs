use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub book_id: u32,
    pub book_name: String,
    pub price: u32,
    pub stock: u32,
}

#[derive(Serialize, Deserialize)]
pub struct BookCategory {
    pub category: String,
    pub published_at: String,
    pub list: Vec<Book>,
}
#[derive(Serialize, Deserialize)]
pub struct BookList {
    pub books: Vec<BookCategory>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestBook {
    pub category: String,
    pub list: Vec<Book>,
}

#[derive(Serialize, Deserialize)]
pub struct BookId {
    pub book_id: u32
}

#[derive(Serialize, Deserialize)]
pub struct DeleteBook {
    pub category: String,
    pub list: Vec<BookId>,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteMessage {
    pub message: String
}