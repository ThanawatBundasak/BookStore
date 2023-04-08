use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub id: i32,
    pub username: String,

}