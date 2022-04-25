pub mod auth;

pub mod handlers;
pub mod db;

use actix_web::{App, web};

use auth::login;


pub(super) fn configure_auth() -> actix_web::Scope{

    web::scope("/auth")
        // .route("/", web::get().to)
        .route("/login", web::post().to(login::login_user))
        .route("/logout", web::post().to(login::logout_user))
}





// fn build_app<T>() ->  App<T>
// {

//     App::new()
// }