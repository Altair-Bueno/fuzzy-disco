use chrono::serde::ts_seconds;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use lazy_static::lazy_static;
use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::outcome::Outcome::{Failure, Forward};
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::Request;
use serde::{Deserialize, Serialize};

/// JWT Time To Live
const TTL_AUTH: i64 = 2;

lazy_static! {
    static ref DECODING_KEY: DecodingKey<'static> =
        DecodingKey::from_base64_secret("hello world").unwrap();
}

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
            None => Outcome::Forward(()),
            Some(Err(_)) => Outcome::Failure((
                Status::BadRequest,
                json!({"status":"BadRequest","message": "Invalid token"}),
            )),
            Some(Ok(x)) if x.claims.is_valid() => Outcome::Success(x.claims),
            _ => Outcome::Failure((
                Status::Unauthorized,
                json!({"status":"Unauthorized","message":"Expired token"}),
            )),
        }
    }
}

impl Token {
    /// Creates a new JWT that is linked to the user ID on the database
    pub fn new(user_id: ObjectId) -> Self {
        let created = Utc::now();
        let expires = created + Duration::days(TTL_AUTH);
        Token {
            user_id,
            created,
            expires,
        }
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
