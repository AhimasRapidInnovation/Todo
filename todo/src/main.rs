#![allow(unused)]

mod todo;

use std::sync::Arc;

use todo::auth::{JwtToken};
use todo::{db::Conn};

use actix_web::{web, App,HttpServer,HttpRequest, Responder};
use dotenv::dotenv;
use env_logger;






#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();

    env_logger::init();
    let mongo_uri = std::env::var("MONGO_URL")?;
    let db = Conn::new(mongo_uri).await?;
    HttpServer::new(
        move || {
                            App::new()
                            .app_data(web::Data::new(db.clone()))
            
        }
    );
    Ok(())
}
