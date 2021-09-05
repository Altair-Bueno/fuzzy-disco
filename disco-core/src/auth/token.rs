use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{DecodingKey, Validation,EncodingKey,Header};
use lazy_static::lazy_static;
use mongodb::bson::oid::ObjectId;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::Value;
use rocket::Request;
use serde::{Deserialize, Serialize};
use crate::auth::result::{AuthError, AuthResult};

/// JWT Time To Live
const TTL_AUTH: i64 = 2;
const SECRET : &str = "hello world";

lazy_static! {
    static ref DECODING_KEY: DecodingKey<'static> =
        DecodingKey::from_base64_secret(SECRET).unwrap();
    static ref ENCODING_KEY: EncodingKey =
        EncodingKey::from_base64_secret(SECRET).unwrap();
}

pub type EncryptedToken = String;

/// Represents a JWT's payload. Visit <https://jwt.io> to learn more about JWT
#[derive(Debug, Serialize, Deserialize, Eq, PartialOrd, PartialEq, Ord)]
pub struct Token {
    user_id: ObjectId,
    created: DateTime<Utc>,
    expires: DateTime<Utc>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = Value;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let validation = Validation::new(Default::default());
        let token = request
            .headers()
            .get("Authorization")
            .next()
            .map(|x| jsonwebtoken::decode::<Token>(x, &DECODING_KEY, &validation));
        match token {
            Some(Ok(x)) if x.claims.is_valid() => Outcome::Success(x.claims),
            _ => Outcome::Forward(()),
        }
    }
}

impl Token {
    /// Creates a new JWT that is linked to the user ID on the database
    pub fn new_encrypted(user_id: ObjectId) -> AuthResult<EncryptedToken> {
        let created = Utc::now();
        let expires = created + Duration::days(TTL_AUTH);
        let token = Token {
            user_id,
            created,
            expires,
        };
        jsonwebtoken::encode(&Header::default(), &token,&ENCODING_KEY)
            .or(Err(AuthError::EncodeError))
    }
    pub fn is_valid(&self) -> bool {
        let expires = self.expires;
        let now = Utc::now();
        expires > now
    }

    pub fn user_id(&self) -> ObjectId {
        self.user_id
    }
    pub fn created(&self) -> DateTime<Utc> {
        self.created
    }
    pub fn expires(&self) -> DateTime<Utc> {
        self.expires
    }
}
