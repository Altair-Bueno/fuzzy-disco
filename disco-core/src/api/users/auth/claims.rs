use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use serde::{Deserialize, Serialize};

use crate::mongo::user::Alias;

/// JWT Time To Live
#[cfg(debug_assertions)]
const TTL_AUTH: i64 = 120;

#[cfg(not(debug_assertions))]
const TTL_AUTH: i64 = 30;

pub type EncryptedToken = String;
pub type ExpiresIn = i64;

/// Represents a JWT's payload. Visit <https://jwt.io> to learn more about JWT
#[derive(Debug, Serialize, Deserialize, Eq, PartialOrd, PartialEq, Ord)]
pub struct TokenClaims {
    sub: Alias,
    exp: i64,
    iat: i64,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TokenClaims {
    type Error = Value;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(authen_header) = request.headers().get_one("Authorization") {
            let authen_str = authen_header.to_string();
            if authen_str.starts_with("Bearer") {
                let token = authen_str[6..authen_str.len()].trim();
                if let Ok(token_data) = jsonwebtoken::decode::<TokenClaims>(
                    token,
                    &DecodingKey::from_secret(include_bytes!("secret.key")),
                    &Validation::default(),
                ) {
                    return Outcome::Success(token_data.claims);
                }
            }
        } else {
            return Outcome::Forward(())
        }

        Outcome::Failure((
            Status::BadRequest,
            json!({"status": Status::BadRequest.reason(), "message": "Invalid token"}),
        ))
    }
}

impl TokenClaims {
    /// Creates a new JWT that is linked to the user ID on the database
    pub fn new_encrypted(alias: Alias) -> (ExpiresIn, EncryptedToken) {
        let created = Utc::now().timestamp();
        let expires = created + TTL_AUTH;

        let claims = TokenClaims {
            sub: alias,
            exp: expires,
            iat: created,
        };

        jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(include_bytes!("secret.key")),
        )
        .map(|x| (TTL_AUTH, x))
        .expect("Token generation failed")
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
