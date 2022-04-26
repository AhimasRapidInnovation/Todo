pub mod auth;
pub mod handlers;
pub mod db;
pub mod structs;


use actix_web::{App, web};

use auth::login;
use structs::{
    UserModel,
    SessionModel,
    SESSION_TABLE,
    USER_TABLE,
};

pub(super) fn configure_auth() -> actix_web::Scope{

    web::scope("/auth")
        // .route("/", web::get().to)
        .route("/login", web::post().to(login::login_user))
        .route("/logout", web::post().to(login::logout_user))
        .route("/create_user", web::post().to(login::create_user))
}


