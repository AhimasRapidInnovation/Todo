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
            info!("result fetched successfully");
            let result = result_set
                .collect::<Vec<_>>()
                .await
                .into_iter()
                .map(Result::unwrap) // never do this on prod
                .collect::<Vec<_>>(); // WARNING : May result in memory overflow
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

pub async fn update(
    conn: web::Data<Conn>,
    current_user: Option<super::JwtToken>,
    update_id: web::Path<String>,
    update: web::Json<super::TodoUpdate>,
) -> impl Responder {
    info!("Update handler called for {:?}", current_user);
    if current_user.is_none() {
        warn!("user session is none");
        return HttpResponse::Unauthorized();
    }
    let update_id = update_id.into_inner();
    let object_id: bson::oid::ObjectId = match bson::oid::ObjectId::parse_str(&update_id) {
        Ok(id) => id,
        Err(e) => {
            warn!("error: Invalid update id");
            return HttpResponse::Conflict();
        }
    };
    match conn
        .collection::<super::TodoItem>(super::TODOS_TABLE)
        .update_one(
            doc! {"_id": object_id},
            doc! {"$set" : {"title": &update.title, "notes": &update.notes}},
            None,
        )
        .await
    {
        Ok(update_res) => {
            assert!(update_res.modified_count == 1);
            return HttpResponse::Ok();
        }
        Err(e) => {
            warn!("error: unable to update due to {}", e);
            return HttpResponse::InternalServerError();
        }
    }
}

pub async fn complete(
    conn: web::Data<Conn>,
    current_user: Option<super::JwtToken>,
    complete_id: web::Path<String>,
) -> impl Responder {
    info!("complete is called for {:?}", current_user);
    if current_user.is_none() {
        warn!("user session is none");
        return HttpResponse::Unauthorized();
    }
    let complete_id = complete_id.into_inner();
    let object_id: bson::oid::ObjectId = match bson::oid::ObjectId::parse_str(&complete_id) {
        Ok(id) => id,
        Err(e) => {
            warn!("error: Invalid complete id");
            return HttpResponse::Conflict();
        }
    };
    match conn
        .collection::<super::TodoItem>(super::TODOS_TABLE)
        .update_one(
            doc! {"_id": object_id},
            doc! {"$set" : {"is_done": true}},
            None,
        )
        .await
    {
        Ok(update_res) => {
            assert!(update_res.modified_count == 1);
            return HttpResponse::Ok();
        }
        Err(e) => {
            warn!("error: unable to complete due to {}", e);
            return HttpResponse::InternalServerError();
        }
    }
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
