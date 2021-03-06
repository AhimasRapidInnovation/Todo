pub mod auth;
pub mod db;
pub mod handlers;
pub mod structs;

use actix_web::{web, App};

use auth::{login, JwtToken};
pub(crate) use db::Conn;

use structs::{
    SessionModel, TodoItem, TodoUpdate, UserModel, SESSION_TABLE, TODOS_TABLE, USER_TABLE,
};

pub(super) fn configure_auth() -> actix_web::Scope {
    web::scope("/auth")
        // .route("/", web::get().to)
        .route("/login", web::post().to(login::login_user))
        .route("/logout", web::post().to(login::logout_user))
        .route("/create_user", web::post().to(login::create_user))
}

pub(super) fn configure_todo() -> actix_web::Scope {
    web::scope("")
        .route("/todos", web::get().to(handlers::todos))
        .route("/create", web::post().to(handlers::create))
        .route("/update/{update_id}", web::post().to(handlers::update))
        .route("/delete/{delete_id}", web::post().to(handlers::delete))
        .route(
            "/complete/{complete_id}",
            web::post().to(handlers::complete),
        )
}
