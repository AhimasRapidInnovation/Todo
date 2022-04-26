

use mongodb::bson::{oid::ObjectId};
use serde::{Serialize,Deserialize};
use bcrypt::{hash, DEFAULT_COST,BcryptError, verify};



pub const USER_TABLE : &'static str = "users";
pub const SESSION_TABLE : &'static str = "session";


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

    pub fn verify_password(&self, plain_password: &str) -> Result<bool, BcryptError>{
        verify(plain_password, self.password.as_str())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SessionModel {
   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub id: Option<ObjectId>,
   pub user_id : String,
   pub token: String
}


impl SessionModel {

    pub fn new(user_id: String, token: String) -> Self {
        Self {id: None, user_id,token}
    }
}


pub struct TodoItem{
    title: String,
    notes: String,
    // user_id
    // is_done
    // created date and time
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usermodel() {
        let user1 = UserModel::new("Thomas Muller".into(), "fc-bayern".into());
        assert!(user1.is_ok());
        let res = user1.as_ref().unwrap().verify_password("fc-bayern".into());
        assert!(res.is_ok());
        assert!(res.unwrap());
    }
}