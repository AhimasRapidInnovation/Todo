#![allow(unused)]

mod todo;

use std::sync::Arc;

use todo::db::Conn;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    env_logger::init();
    let mongo_uri = std::env::var("MONGO_URL")?;
    let db = Conn::new(mongo_uri).await?;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            // .route("/", web::get().to(|| async move { HttpResponse::Ok() }))
            .service(todo::configure_auth())
            .service(todo::configure_todo())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;
    Ok(())
}
