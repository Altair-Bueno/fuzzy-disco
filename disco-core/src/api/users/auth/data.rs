use serde::{Deserialize, Serialize};

use crate::mongo::user::{User, UserError};
use crate::mongo::IntoDocument;


use chrono::{DateTime, Duration, Utc};
use lazy_static::lazy_static;
use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::Request;

use crate::api::users::auth::result::{AuthError, AuthResult};
use crate::mongo::user::Alias;
use jsonwebtoken::{DecodingKey, Validation, Header, EncodingKey};

/// JWT Time To Live
const TTL_AUTH: i64 = 5;


pub type EncryptedToken = String;
pub type ExpireDate = i64;

/// Represents a JWT's payload. Visit <https://jwt.io> to learn more about JWT
#[derive(Debug, Serialize, Deserialize, Eq, PartialOrd, PartialEq, Ord,Clone)]
pub struct Claims {
    sub: Alias,
    exp: i64,
    iat: i64,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = Value;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(authen_header) = request.headers().get_one("Authorization") {
            let authen_str = authen_header.to_string();
            if authen_str.starts_with("Bearer") {
                let token = authen_str[6..authen_str.len()].trim();
                if let Ok(token_data) =
                    jsonwebtoken::decode::<Claims>(
                        &token,
                        &DecodingKey::from_secret(include_bytes!("../../../../secret.key")),
                        &Validation::default()
                    ) {
                        return Outcome::Success(token_data.claims)
                    }
                }
            }

        Outcome::Failure((
            Status::BadRequest,
            json!({"status": Status::BadRequest.reason(), "message": "Invalid token"})
        ))
    }
}

impl Claims {
    /// Creates a new JWT that is linked to the user ID on the database
    pub fn new_encrypted(alias: Alias) -> AuthResult<(ExpireDate, EncryptedToken)> {
        let created = Utc::now();
        let expires = created + Duration::minutes(TTL_AUTH);

        let claims = Claims {
            sub: alias,
            exp: expires.timestamp(),
            iat: created.timestamp()
        };
        let token = jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(include_bytes!("../../../../secret.key"))).unwrap();

        Ok((expires.timestamp(),token))
    }

    pub fn created(&self) -> i64 {
        self.iat
    }
    pub fn expires(&self) -> i64 {
        self.exp
    }
    pub fn alias(&self) -> &Alias {
        &self.sub
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserSingUp<'a> {
    alias: &'a str,
    email: &'a str,
    password: &'a str,
}

impl IntoDocument<User> for UserSingUp<'_> {
    type Err = UserError;

    fn validate(self) -> Result<User, Self::Err> {
        let UserSingUp {
            alias,
            email,
            password,
        } = self;
        let alias = alias.parse()?;
        let email = email.parse()?;
        let password = password.parse()?;
        Ok(User::new(alias, email, password))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogInEmail<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogInAlias<'a> {
    pub alias: &'a str,
    pub password: &'a str,
}

#[cfg(test)]
mod tes {
    use crate::api::users::auth::data::Claims;
    use jsonwebtoken::{DecodingKey,Validation};

    #[test]
    pub fn test() {
        let (_,token) = Claims::new_encrypted("Temp".parse().unwrap()).unwrap();
        println!("{:?}",token);
        let decript =  jsonwebtoken::decode::<Claims>(
            &token,
            &DecodingKey::from_secret(include_bytes!("../../../../secret.key")),
            &Validation{validate_exp: false, ..Validation::default()}
        ).unwrap();
        println!("{:?}",decript)

    }
}