use std::net::IpAddr;

use mongodb::bson::oid::ObjectId;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};

use crate::mongo::user::{User, UserError};
use crate::mongo::IntoDocument;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSingUp<'a> {
    pub alias: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

impl IntoDocument<User> for UserSingUp<'_> {
    type Err = UserError;

    fn validate(&self) -> Result<User, Self::Err> {
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

pub struct IpAdd {
    pub ip: IpAddr,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IpAdd {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.client_ip() {
            None => Outcome::Forward(()),
            Some(ip) => Outcome::Success(IpAdd { ip }),
        }
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
    // session token
    pub id: ObjectId,
    // date
    pub date: mongodb::bson::DateTime,
    // joined field
    pub users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshJWT {
    pub refresh_token: ObjectId,
}
