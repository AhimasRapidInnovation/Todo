use actix_web::{get,Responder};



#[get("/login")]
async fn login() -> impl Responder 
{

    "Logging in ".to_string()
}