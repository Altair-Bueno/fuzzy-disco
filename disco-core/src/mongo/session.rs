use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

use crate::mongo::traits::Document;
use crate::mongo::user::Alias;

/// Contains information about a user login session (aka refresh token). Each
/// time the server recives a valid `POST /api/user/login`, a new session will
/// be created on the server. This allows the user to refresh its JWT auth token
/// without use of username and password
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct Session {
    // Session token
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    // subject alias
    user_alias: Alias,
    // where
    ip: Option<String>,
    // date
    date: DateTime,
}

impl Session {
    /// Generates a new session token that is linked to the user's alias
    pub fn new(user_alias: Alias, ip: Option<String>) -> Session {
        Session {
            id: None,
            user_alias,
            ip,
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
    pub fn user_alias(&self) -> &Alias {
        &self.user_alias
    }

    pub fn ip(&self) -> &Option<String> {
        &self.ip
    }
}

impl Document for Session {}
