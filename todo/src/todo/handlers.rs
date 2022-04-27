
use actix_web::{Responder, web, HttpRequest, HttpResponse};
use serde_json::json;
use log::{warn, info};

use super::db::Conn;

pub async fn todos() -> impl Responder{
    "list here"
}


pub async fn create(conn: web::Data<Conn>,item: web::Json<super::TodoItem>, current_user: Option<super::JwtToken>) -> impl Responder {
    
    info!("create user is invoked");
    if current_user.is_none() {
        warn!("user session is none");
        return HttpResponse::Unauthorized()
            .json(json!({"status": "Failure", "Err": "Unauthorized"}));
    }
    let mut item = item.0;
    item.created_at = Some(bson::DateTime::from_chrono(chrono::Utc::now()));
    item.user_id = Some(current_user.as_ref().unwrap().user_id.to_owned());
    item.is_done = Some(false);
    match conn.collection(super::TODOS_TABLE).insert_one(item,None).await {
        Ok(inserted) => {
            info!("ToDo created successfully");
            return HttpResponse::Ok().json(json!({"status":"success"}))
        },
        Err(e) => {
            warn!("error: unable to create todos {}", e);
            return HttpResponse::InternalServerError().json(json!({"status": "Failed", "Err": e.to_string()}));
        }
    }
}

pub async fn update() -> impl Responder {
    "Updated sucessfully"
}

pub async fn delete() -> impl Responder{
    "Deleted sucessfully"
}