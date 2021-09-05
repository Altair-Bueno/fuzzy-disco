use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use rocket::request::FromParam;
use rocket::serde::json::serde_json::json;
use serde::Deserialize;
use serde::Serialize;

/// Wrapper for [`ObjectId`](mongodb::bson::oid::ObjectId). Implements the
/// [`FromParam`](rocket::request::FromParam) trait.
#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
pub struct Id(ObjectId);

impl FromParam<'_> for Id {
    type Error = rocket::serde::json::Value;

    fn from_param(param: &str) -> Result<Self, Self::Error> {
        ObjectId::from_str(param)
            .map(Id)
            .map_err(|_| json!({"Error": "Invalid ID"}))
    }
}

impl Id {
    pub fn extract(self) -> ObjectId {
        self.0
    }
}
