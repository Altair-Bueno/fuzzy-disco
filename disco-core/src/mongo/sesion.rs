use crate::mongo::traits::Document;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use crate::mongo::user::Alias;

/// Contains information about a user login sesion. Each time rocket recives a
/// valid `POST /api/user/login`, a new Sesion will be created on the server.
/// This allows the user to refresh its JWT auth token without use of username
/// and password. It also allows the user to log out remotly
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct Sesion {
    // sesion token
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    // subject object id
    user_alias: Alias,
    // date
    date: DateTime,
}

impl Sesion {
    pub fn new(user_alias: Alias) -> Sesion {
        Sesion {
            id: None,
            user_alias,
            date: DateTime::now(),
        }
    }

    pub fn id(&self) -> Option<ObjectId> {
        self.id
    }
    pub fn sub(&self) -> &Alias {
        &self.user_alias
    }
    pub fn date(&self) -> DateTime {
        self.date
    }
}

impl Document for Sesion {}
