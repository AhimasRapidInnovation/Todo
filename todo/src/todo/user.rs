use serde::{Serialize,Deserialize};



struct Login{
    username : String,
    password :  String,
    
}



#[derive(Serialize,Deserialize)]
struct Session 
{
    token :  String,
    user_id : String,
}


