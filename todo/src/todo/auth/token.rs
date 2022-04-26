use actix_web::{error, error::Error, web, FromRequest, HttpResponse};
use futures::executor::block_on;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, Token, VerifyWithKey};
use log::{debug, warn};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::{future::Future, pin::Pin};

use super::{Conn, SessionModel, SESSION_TABLE};

type HamcSha256 = Hmac<Sha256>;

// try to get from env
const SECRET: &'static str = "super-secret";
const TOKEN_HEADER: &'static str = "bearer-token";

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct JwtToken {
    pub user_id: String,
    pub tok: String,
}

impl FromRequest for JwtToken {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let conn = req.app_data::<web::Data<Conn>>().unwrap();
        let session_collection = conn.collection::<SessionModel>(SESSION_TABLE);
        let header = req.headers();
        if !header.contains_key(TOKEN_HEADER) {
            return Box::pin(
                async move { Err(error::ErrorUnauthorized("user is not authorized")) },
            );
        }
        let token = header
            .get(TOKEN_HEADER)
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        let jwt_token = JwtToken::decode(token);
        // If session does not exist give authorization error
        // do not use block_on
        match block_on(
            session_collection.find_one(doc! {"user_id": jwt_token.user_id.clone()}, None),
        ) {
            Ok(item) => {
                // if session already exists then leave it else
                if item.is_none() {
                    return Box::pin(async move {
                        Err(error::ErrorUnauthorized(json!({"Err":"Unauthorized"})))
                    });
                }
                let item = item.unwrap();
                if item.token != jwt_token.tok {
                    return Box::pin(async move {
                        Err(error::ErrorUnauthorized(json!({"Err":"Invalid Token"})))
                    });
                }
            }
            Err(e) => {
                warn!("error: database error {}", e);
                return Box::pin(async move {
                    Err(error::ErrorInternalServerError(
                        json!({"Err":"internal server error "}),
                    ))
                });
            }
        }
        Box::pin(async move { Ok(jwt_token) })
    }
}

struct TokenGenerator {
    secret: String,
}

impl JwtToken {
    pub fn new(user_id: String) -> Self {
        let secret_key = HamcSha256::new_from_slice(SECRET.as_bytes()).unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("user_id", user_id.clone());
        let tok = claims.sign_with_key(&secret_key).unwrap();
        Self {
            user_id: user_id,
            tok,
        }
    }

    pub fn decode(token: String) -> Self {
        let secret_key = HamcSha256::new_from_slice(SECRET.as_bytes()).unwrap();
        let claims: BTreeMap<String, String> = token.verify_with_key(&secret_key).unwrap();
        Self {
            user_id: claims["user_id"].to_string(),
            tok: token.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt() {
        let user_id = "1234";
        let token = JwtToken::new(user_id.clone().into());
        eprintln!("token {:?}", token);
        let gen_tok = token.tok.clone();
        let new_tok = JwtToken::decode(gen_tok);
        assert_eq!(token, new_tok);
    }
}
