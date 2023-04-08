use actix_web::{App, HttpServer,};
use env_logger::Env;
pub mod routes;
pub mod handlers;
pub mod models;
use crate::routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
   HttpServer::new(|| {
       App::new()
            .configure(profile_route::config)
            .configure(list_route::config)
            .configure(add_route::config)
            .configure(update_route::config)
            .configure(delete_route::config)
   })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}

