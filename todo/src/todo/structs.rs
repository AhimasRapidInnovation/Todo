use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub const USER_TABLE: &'static str = "users";
pub const SESSION_TABLE: &'static str = "session";
pub const TODOS_TABLE : &str = "todos";




#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UserModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub password: String,
}

impl UserModel {
    pub fn new(name: String, password: String) -> Result<Self, BcryptError> {
        let hashed_password = hash(password.as_str(), DEFAULT_COST)?;
        Ok(Self {
            id: None,
            name: name,
            password: hashed_password,
        })
    }

    pub fn verify_password(&self, plain_password: &str) -> Result<bool, BcryptError> {
        verify(plain_password, self.password.as_str())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SessionModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub token: String,
}

impl SessionModel {
    pub fn new(user_id: String, token: String) -> Self {
        Self {
            id: None,
            user_id,
            token,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    // Title of the to do item
    pub title: String,
    
    // Actual content of the todo 
    pub notes: String,
    
    // For which user usually ObjectId.to_hex()
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_id : Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_done  : Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at : Option<bson::DateTime>
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
