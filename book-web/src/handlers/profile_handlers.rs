use actix_web::{get, Responder, HttpResponse};
use crate::models::profile::Profile;
use serde_json::json;

#[get("/profile")]
async fn get_id() -> impl Responder{
    let id: i32 = 20;
    let profile = vec![
        Profile {
            id: id,
            username: "Pond".to_string()
        }
    ];
    let response_body = json!(profile);

    HttpResponse::Ok().json(response_body)
}