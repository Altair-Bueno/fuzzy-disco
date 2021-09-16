use serde::{Deserialize, Serialize};

use crate::mongo::post::Post;
use crate::mongo::user::User;
use crate::mongo::visibility::Visibility;
use std::str::FromStr;
use rocket::request::FromParam;
use chrono::{Utc, DateTime};
use rocket::form::{FromFormField, ValueField};
use rocket::form;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiUserResponse {
    pub alias: String,
    pub description: Option<String>,
    pub avatar: Option<String>,
}

impl From<User> for ApiUserResponse {
    fn from(u: User) -> Self {
        ApiUserResponse {
            alias: u.alias().to_string(),
            description: u.description().clone().map(|x| x.to_string()),
            avatar: u.avatar().map(|x| x.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiPostResponse {
    id: Option<String>,
    title: String,
    caption: String,
    author: String,
    audio: String,
    photo: String,
    visibility: Visibility,
    creation_date: String,
}

impl From<Post> for ApiPostResponse {
    fn from(p: Post) -> Self {
        ApiPostResponse {
            id: p.id().map(|x| x.to_string()),
            title: p.title().to_string(),
            caption: p.caption().to_string(),
            author: p.author().to_string(),
            audio: p.audio().to_string(),
            photo: p.photo().to_string(),
            visibility: p.visibility().clone(),
            creation_date: p.creation_date().to_string(),
        }
    }
}


#[derive(Serialize,Deserialize,Debug)]
pub struct ObjectIdWrapper(mongodb::bson::oid::ObjectId);

impl ObjectIdWrapper {
    pub fn extract(self) -> mongodb::bson::oid::ObjectId {
        let ObjectIdWrapper(oid) = self;
        oid
    }
}
impl FromStr for ObjectIdWrapper {
    type Err = mongodb::bson::oid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<mongodb::bson::oid::ObjectId>()
            .map(|x| ObjectIdWrapper(x))
    }
}

impl<'a> FromParam<'a> for ObjectIdWrapper {
    type Error = mongodb::bson::oid::Error;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        ObjectIdWrapper::from_str(param)
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct ApiDate(mongodb::bson::DateTime);

impl ApiDate {
    pub fn extract(&self) -> mongodb::bson::DateTime {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for ApiDate {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        let res = ApiDate::from_str(field.value);
        match res {
            Ok(x) => Ok(x),
            Err(_) => Err(form::Error::validation("err"))?
        }
    }
}
impl<'a> FromParam<'a> for ApiDate {
    type Error = chrono::ParseError;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        ApiDate::from_str(param)
    }
}
impl FromStr for ApiDate {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date: DateTime<Utc> = s.parse()?;
        Ok(ApiDate(mongodb::bson::DateTime::from_chrono(date)))
    }
}