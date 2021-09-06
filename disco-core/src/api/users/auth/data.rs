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

use crate::api::users::auth::result::{AuthError, AuthResult};
use crate::mongo::user::Alias;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

use std::io::Cursor;

use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogInRefreshToken<'a> {
    pub refresh_token: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinedRefreshToken {
    // sesion token
    pub id: ObjectId,
    // date
    pub date: mongodb::bson::DateTime,
    // joined field
    pub users: Vec<User>,
}
