use super::{Conn, SESSION_TABLE, USER_TABLE};
use actix_web::{post, web, HttpResponse, Responder};
use futures::stream::{StreamExt, TryStreamExt};
use log::{debug, info, warn};
use mongodb::bson::doc;
use serde::Deserialize;
use serde_json::json;

pub(crate) async fn login_user(
    conn: web::Data<Conn>,
    user: web::Json<super::LoginUser>,
) -> impl Responder {
    info!("Login user is invoked for {}", user.username);
    let user_collection = conn.collection::<crate::todo::UserModel>(USER_TABLE);
    // Check is user already exist ?
    let user_res = match user_collection
        .find(doc! {"name": user.username.clone()}, None)
        .await
    {
        Ok(cur) => cur.collect::<Vec<_>>().await,
        Err(e) => {
            warn!("error:  {}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "Internal Server Error", "Err": e.to_string()}));
        }
    };
    if user_res.is_empty() {
        warn!("error: user does not exist");
        return HttpResponse::NotFound()
            .json(json!({"status": "Not Found", "Err": "User does not exist"}));
    } else if user_res.len() > 1 {
        warn!("error: Multiple user exist for {:?}", user.username);
        return HttpResponse::Conflict().json(json!({"status": "Conflict"}));
    }
    let db_user = match &user_res[0] {
        Ok(user_model) => user_model,
        Err(e) => {
            warn!("error: {}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "Internal Server Error", "Err": e.to_string()}));
        }
    };
    match db_user.verify_password(user.password.as_str()) {
        Ok(val) => {
            if !val {
                warn!("error: invalid password");
                return HttpResponse::Unauthorized()
                    .json(json!({"status": "Failed", "Err": "Invalid Password"}));
            } else {
                let user_id = db_user.id.unwrap().to_hex();
                let token = super::JwtToken::new(user_id.clone());
                info!("Logged in sucessful");
                let session_collection = conn.collection::<super::SessionModel>(SESSION_TABLE);
                let _ = session_collection
                    .delete_many(doc! {"user_id": &user_id}, None)
                    .await;
                match session_collection
                    .insert_one(super::SessionModel::new(user_id, token.tok.clone()), None)
                    .await
                {
                    Err(e) => {
                        warn!("error: failed to create the session {}", e);
                        return HttpResponse::InternalServerError()
                            .json(json!({"status": "Internal Server Error", "Err": e.to_string()}));
                    }
                    _ => {
                        info!("session created sucessfully ");
                    }
                }
                return HttpResponse::Ok().json(json!({"bearer-token" : token.tok}));
            }
        }
        Err(e) => {
            warn!("error: bcrypt error {}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "Internal Server Error", "Err": e.to_string()}));
        }
    }
}

pub(crate) async fn logout_user(
    conn: web::Data<Conn>,
    current_user: Option<super::JwtToken>,
) -> impl Responder {
    if current_user.is_none() {
        warn!("user session is none");
        return HttpResponse::Unauthorized()
            .json(json!({"status": "Failure", "Err": "Unauthorized"}));
    }
    let current_user = current_user.unwrap();
    let session_collection = conn.collection::<super::SessionModel>(SESSION_TABLE);
    match session_collection
        .delete_one(doc! {"user_id": current_user.user_id.clone()}, None)
        .await
    {
        Ok(delete_res) => {
            assert!(delete_res.deleted_count == 1);
        }
        Err(e) => {
            warn!("error: unable to delete session {}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "Internal Server Error", "Err": e.to_string()}));
        }
    };
    HttpResponse::Ok().json(json!({"status":"success"}))
}

pub(crate) async fn create_user(
    conn: web::Data<Conn>,
    user: web::Json<super::CreateUser>,
) -> impl Responder {
    info!("create_user is invoked with {:?}", user);
    if user.password != user.confirm_password {
        return HttpResponse::Conflict()
            .json(json!({"status":"Failed", "message": "password does not match"}));
    }
    let user_collection = conn.collection::<crate::todo::UserModel>(USER_TABLE);
    let has_user = match user_collection
        .find_one(doc! {"name": user.username.clone()}, None)
        .await
    {
        Ok(item) => item,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(json!({"status": "Internal Server Error", "Err": e.to_string()}))
        }
    };
    if has_user.is_some() {
        warn!("User already created ");
        return HttpResponse::Conflict()
            .json(json!({"status": "Failure", "message": "user already exist"}));
    }

    info!("creating user");
    let new_user = match super::UserModel::new(user.username.clone(), user.password.clone()) {
        Ok(user) => user,
        Err(e) => {
            warn!("unable to create the new user {}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "Internal Server Error", "Err": e.to_string()}));
        }
    };
    let res = match user_collection.insert_one(new_user, None).await {
        Ok(inserted) => inserted.inserted_id,
        Err(e) => {
            warn!("Unable to create the user {}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "Internal Server Error", "Err": e.to_string()}));
        }
    };

    HttpResponse::Created().json(json!({"status": "success"}))
}
