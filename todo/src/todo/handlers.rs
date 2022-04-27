use actix_web::{web, HttpRequest, HttpResponse, Responder};
use bson::doc;
use log::{info, warn};
use serde_json::json;

use super::db::Conn;
use futures::stream::{StreamExt, TryStreamExt};

pub async fn todos(conn: web::Data<Conn>, current_user: Option<super::JwtToken>) -> impl Responder {
    if current_user.is_none() {
        warn!("user session is none");
        return HttpResponse::Unauthorized()
            .json(json!({"status": "Failure", "Err": "Unauthorized"}));
    }
    match conn
        .collection::<super::TodoItem>(super::TODOS_TABLE)
        .find(
            doc! {"user_id" : current_user.as_ref().unwrap().user_id.clone()},
            None,
        )
        .await
    {
        Ok(result_set) => {
            // info!("");
            // let result = match result_set.collect::<().await{
            //     Ok(res) => res,
            //     Err(e) => {
            //         return HttpResponse::InternalServerError().json(json!({"status": "Failure"}))
            //     }
            // };
            let result = result_set
                .collect::<Vec<_>>()
                .await
                .into_iter()
                .map(Result::unwrap)
                .collect::<Vec<_>>();
            return HttpResponse::Ok().json(json!(result));
        }
        Err(e) => {
            warn!("error: unable to create todos {}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "Failed", "Err": e.to_string()}));
        }
    }
}

pub async fn create(
    conn: web::Data<Conn>,
    item: web::Json<super::TodoItem>,
    current_user: Option<super::JwtToken>,
) -> impl Responder {
    info!("create user is invoked for {:?}", current_user);
    if current_user.is_none() {
        warn!("user session is none");
        return HttpResponse::Unauthorized()
            .json(json!({"status": "Failure", "Err": "Unauthorized"}));
    }
    let mut item = item.0;
    item.created_at = Some(bson::DateTime::from_chrono(chrono::Utc::now()));
    item.user_id = Some(current_user.as_ref().unwrap().user_id.to_owned());
    item.is_done = Some(false);
    match conn
        .collection(super::TODOS_TABLE)
        .insert_one(item, None)
        .await
    {
        Ok(inserted) => {
            info!("ToDo created successfully");
            return HttpResponse::Ok().json(json!({"status":"success"}));
        }
        Err(e) => {
            warn!("error: unable to create todos {}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "Failed", "Err": e.to_string()}));
        }
    }
}

pub async fn update(current_user: Option<super::JwtToken>) -> impl Responder {
    info!("Update handler called for {:?}", current_user);
    "Updated sucessfully"
}

pub async fn delete(
    conn: web::Data<Conn>,
    delete_id: web::Path<String>,
    current_user: Option<super::JwtToken>,
) -> impl Responder {
    info!("create user is invoked for {:?}", current_user);
    if current_user.is_none() {
        warn!("user session is none");
        return HttpResponse::Unauthorized();
    }
    let delete_id = delete_id.into_inner();
    let object_id: bson::oid::ObjectId = match bson::oid::ObjectId::parse_str(&delete_id) {
        Ok(id) => id,
        Err(e) => {
            warn!("error: Invalid delete id");
            return HttpResponse::Conflict();
        }
    };
    match conn
        .collection::<super::TodoItem>(super::TODOS_TABLE)
        .find_one_and_delete(doc! {"_id": object_id}, None)
        .await
    {
        Ok(res) => {
            if res.is_none() {
                warn!("error: delete result is none");
                return HttpResponse::Conflict();
            } else {
                info!("deleted sucessfully");
                return HttpResponse::Ok();
            }
        }
        Err(e) => {
            warn!("error: unable to delete due to {}", e);
            return HttpResponse::InternalServerError();
        }
    }
}
