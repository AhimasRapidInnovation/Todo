


use std::collections::BTreeMap;
use sha2::Sha256;
use hmac::{ Hmac,Mac};
use jwt::{SignWithKey, VerifyWithKey, Token};
use actix_web::{error::Error, FromRequest};

use std::{pin::Pin, future::Future};


type HamcSha256 = Hmac<Sha256>; 



// try to get from env
const SECRET : &'static str = "super-secret";

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct JwtToken {
    user_id : String,
    tok :  String,
}


impl FromRequest for JwtToken {

   type Error = Error;
   type Future = Pin<Box<dyn Future<Output = Result<Self,Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future 
    {
        //  TODO
        // 
        Box::pin( async move {
            Ok(JwtToken::new("1234".into()))
        })
    }




}

struct TokenGenerator {
    secret :  String,
}

impl JwtToken {


    pub fn new(user_id: String) -> Self {

        let secret_key = HamcSha256::new_from_slice(SECRET.as_bytes()).unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("user_id", user_id.clone());
        let tok = claims.sign_with_key(&secret_key).unwrap();
        Self {user_id: user_id, tok:tok} 
    }

    pub fn decode(token: String) -> Self {
        let secret_key = HamcSha256::new_from_slice(SECRET.as_bytes()).unwrap();
        let claims : BTreeMap<String,String> = token.verify_with_key(&secret_key).unwrap();
        Self {user_id: claims["user_id"].to_string(), tok: token.to_string()}

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