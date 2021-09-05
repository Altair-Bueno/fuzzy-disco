use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::Request;
use serde::{Deserialize, Serialize};

use crate::api::users::auth::result::{AuthError, AuthResult};
use crate::mongo::user::Alias;

/// JWT Time To Live
const TTL_AUTH: i64 = 5;
const SECRET: &str = "hello world";

lazy_static! {
    static ref DECODING_KEY: DecodingKey<'static> = DecodingKey::from_secret(SECRET.as_ref());
    static ref ENCODING_KEY: EncodingKey = EncodingKey::from_secret(SECRET.as_ref());
}

pub type EncryptedToken = String;
pub type ExpireDate = DateTime<Utc>;

/// Represents a JWT's payload. Visit <https://jwt.io> to learn more about JWT
#[derive(Debug, Serialize, Deserialize, Eq, PartialOrd, PartialEq, Ord)]
pub struct Token {
    alias: Alias,
    created: DateTime<Utc>,
    expires: DateTime<Utc>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = Value;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request
            .headers()
            .get("Authorization")
            .next()
            .map(|x| jsonwebtoken::decode::<Token>(x, &DECODING_KEY, &Validation::default()));
        match token {
            Some(Ok(x)) if x.claims.is_valid() => Outcome::Success(x.claims),
            Some(Ok(_)) => Outcome::Failure((
                Status::new(440),
                json!({"status": "LoginTimeout", "message": "Sesion has expired"}),
            )),
            Some(Err(_)) => Outcome::Failure((
                Status::BadRequest,
                json!({"status": "BadRequest","message": "Invalid token"}),
            )),
            _ => Outcome::Forward(()),
        }
    }
}

impl Token {
    /// Creates a new JWT that is linked to the user ID on the database
    pub fn new_encrypted(alias: Alias) -> AuthResult<(ExpireDate, EncryptedToken)> {
        let created = Utc::now();
        let expires = created + Duration::minutes(TTL_AUTH);
        let token = Token {
            alias,
            created,
            expires,
        };
        jsonwebtoken::encode(&Header::default(), &token, &ENCODING_KEY)
            .map(|x| (expires,x))
            .or(Err(AuthError::EncodeError))
    }
    pub fn is_valid(&self) -> bool {
        let expires = self.expires;
        let now = Utc::now();
        expires > now
    }


    pub fn created(&self) -> DateTime<Utc> {
        self.created
    }
    pub fn expires(&self) -> DateTime<Utc> {
        self.expires
    }
    pub fn alias(&self) -> &Alias {
        &self.alias
    }
}
