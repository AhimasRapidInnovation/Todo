

use mongodb::bson::{oid::ObjectId};
use serde::{Serialize,Deserialize};
use bcrypt::{hash, DEFAULT_COST,BcryptError};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UserModel {
   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub id: Option<ObjectId>,
   pub name: String,
   pub password: String,
}

impl UserModel {

    pub fn new(name: String, password: String) -> Result<Self,BcryptError>  
    {
        let hashed_password = hash(password.as_str(), DEFAULT_COST)?;
        Ok(Self{id: None, name: name, password: hashed_password})
    }
}

pub struct TodoItem{
    title: String,
    notes: String,
    // user_id
    // is_done
    // created date and time
}