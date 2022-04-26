use serde::{Deserialize, Serialize};

// # LoginUser
// Login to the system with this structure
#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Serialize, Deserialize)]
struct Session {
    token: String,
    user_id: String,
}
