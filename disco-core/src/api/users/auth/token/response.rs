use std::io::Cursor;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket::serde::json::serde_json::json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    expires_in: i64,
    refresh_token: String,
}

impl TokenResponse {
    pub fn new(expires_in: i64, refresh_token: String, access_token: String) -> TokenResponse {
        TokenResponse {
            access_token,
            expires_in,
            refresh_token,
        }
    }
}

impl<'r, 'o> Responder<'r, 'static> for TokenResponse {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let value = json!(
            {
                "access_token": self.access_token,
                "expires_in": self.expires_in,
                "refresh_token": self.refresh_token,
                "token_type": "Bearer",
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
