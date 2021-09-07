use rocket::response::status::Custom;
use rocket::serde::json::Value;
use thiserror::Error;
use rocket::response::Responder;
use rocket::{Request, response, Response};
use rocket::response::status;
use rocket::http::{Status, Header};
use rocket::serde::json::serde_json::json;
use std::io::Cursor;

pub type ApiResult = Custom<Value>;



#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Couldn't retrive data from database")]
    DatabaseError(#[from] mongodb::error::Error),
    #[error(transparent)]
    InvalidUser(#[from] crate::mongo::user::UserError),
    #[error(transparent)]
    InvalidPost(#[from] crate::mongo::post::PostError),
    #[error("{0} taken")]
    Conflict(&'static str),
    #[error("{0}")]
    Unauthorized(&'static str),
    #[error("{0}")]
    InternalServerError(&'static str),
    #[error("{0} not found")]
    NotFound(&'static str)
}
impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let status = match self {
            ApiError::DatabaseError(_) | ApiError::InternalServerError(_) => Status::InternalServerError,
            ApiError::InvalidUser(_) | ApiError::InvalidPost(_) => Status::BadRequest,
            ApiError::Conflict(_) => Status::Conflict,
            ApiError::Unauthorized(_)=> Status::Unauthorized,
            ApiError::NotFound(_) => Status::NotFound
        };
        let body = json!({
            "status": status.reason(),
            "message": format!("{}",self)
        }).to_string();
        Response::build()
            .status(status)
            .raw_header("Content-Type","application/json")
            .sized_body(body.len(),Cursor::new(body))
            .ok()
    }
}
