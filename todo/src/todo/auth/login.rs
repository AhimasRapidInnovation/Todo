use actix_web::{post,Responder, web, HttpResponse};
use super::{Conn};
use serde::Deserialize;
use serde_json::json;
use mongodb::bson::doc;
use log::{info, debug,warn};

const USER_TABLE : &'static str = "users";


pub(crate) async fn login_user(conn : web::Data<Conn>, user: web::Json<super::Login>) -> impl Responder 
{
    eprintln!("user {:?}", user);
    "Logging in ".to_string()
}

pub(crate) async fn logout_user(conn : web::Data<Conn>) -> impl Responder 
{

    "Logging out ".to_string()
}

pub(crate) async fn create_user(conn : web::Data<Conn>, user: web::Json<super::CreateUser>) -> impl Responder
{
    info!("create_user is invoked with {:?}", user);
    if user.password != user.confirm_password{
        return HttpResponse::Conflict().json(json!({"status":"Failed", "message": "password does not match"}))
    }
    let user_collection = conn.collection::<crate::todo::UserModel>(USER_TABLE);
    let has_user = match user_collection.find_one(doc!{"name": user.username.clone()}, None).await{
        Ok(item) => item,
        Err(e) => return HttpResponse::InternalServerError().json(json!({"status": "Internal Server Error", "Err": e.to_string()}))
    };
    if has_user.is_some(){
        warn!("User already created ");
        return HttpResponse::Conflict().json(json!({"status": "Failure", "message": "user already exist"}));
    } 

    info!("creating user");
    let new_user = match super::UserModel::new(user.username.clone(), user.password.clone()){
        Ok(user) => user,
        Err(e) => {
            warn!("unable to create the new user {}", e);
            return HttpResponse::InternalServerError().json(json!({"status": "Internal Server Error", "Err": e.to_string()}));
        }
    };
    let res = match user_collection.insert_one(new_user, None).await{
        Ok(inserted) => inserted.inserted_id,
        Err(e) => {
            warn!("Unable to create the user {}", e);
            return HttpResponse::InternalServerError().json(json!({"status": "Internal Server Error", "Err": e.to_string()}));
        }
    };

    HttpResponse::Created().json(json!({"status": "success"}))
}