pub mod auth;
pub mod db;
pub mod handlers;
pub mod structs;

use actix_web::{web, App};

use auth::login;
use structs::{SessionModel, UserModel, SESSION_TABLE, USER_TABLE};

pub(super) fn configure_auth() -> actix_web::Scope {
    web::scope("/auth")
        // .route("/", web::get().to)
        .route("/login", web::post().to(login::login_user))
        .route("/logout", web::post().to(login::logout_user))
        .route("/create_user", web::post().to(login::create_user))
}
