use serde::{Deserialize, Serialize};

use crate::mongo::user::{User, UserError};
use crate::mongo::IntoDocument;

use chrono::{DateTime, Duration, Utc};
use lazy_static::lazy_static;
use mongodb::bson::oid::ObjectId;
use rocket::http::{Status};
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;

use crate::api::users::auth::result::{AuthError, AuthResult};
use crate::mongo::user::Alias;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

use std::io::Cursor;

use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::ContentType;
#[derive(Debug,Serialize,Deserialize)]
pub struct TokenResponse {
    access_token: String,
    expires_in: i64,
    refresh_token: String,
}

impl TokenResponse {
    pub fn new(expires_in:i64,refresh_token:String,access_token:String) -> TokenResponse {
        TokenResponse {
            access_token,
            expires_in,
            refresh_token,
        }
    }
}

impl <'r,'o>Responder<'r,'static> for TokenResponse {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let value = json!(
            {
                "access_token": self.access_token,
                "token_type": "Bearer",
                "expires_in": self.expires_in,
                "refresh_token": self.refresh_token,
                "scope": "User login"
            }
        );
        let body = rocket::serde::json::serde_json::to_string(&value).unwrap();
        Response::build()
            .status(Status::Ok)
            .header(ContentType::JSON)
            .raw_header("Cache-Control", "no-store")
            .raw_header("Pragma", "no-cache")
            .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}