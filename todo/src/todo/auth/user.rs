use serde::{Serialize,Deserialize};


#[derive(Debug, Deserialize)]
pub struct Login{
    pub username : String,
    pub password :  String,
}


#[derive(Serialize,Deserialize)]
struct Session 
{
    token :  String,
    user_id : String,
}


