use actix_web::{post,Responder, web};
use super::Conn;
use serde::Deserialize;




pub(crate) async fn login_user(conn : web::Data<Conn>, user: web::Json<super::Login>) -> impl Responder 
{
    eprintln!("user {:?}", user);
    "Logging in ".to_string()
}

pub(crate) async fn logout_user(conn : web::Data<Conn>) -> impl Responder 
{

    "Logging out ".to_string()
}