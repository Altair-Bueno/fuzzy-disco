use serde::{Deserialize, Serialize};
use validator::Validate;
use mongodb::bson::oid::ObjectId;
use crate::mongo::media::class::Class;
use crate::mongo::media::status::Status;

const EXPIRE_IN: i64 = 30;

// TODO better doc
/// A Media instance contains information about how to locate a resource on the
/// server as an absolute path.
#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize,Validate)]
pub struct Media {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    url: String,
    class: Class,
    status: Status
}

impl Media {
    pub fn new(url:&str,class:Class) -> Media {
        let expires = chrono::Utc::now() + chrono::Duration::minutes(EXPIRE_IN);
        Media {
            id: None,
            url:url.to_string(),
            class,
            status: Status::TTL {
                ttl: mongodb::bson::DateTime::from_chrono(expires),
            }
        }
    }

    pub fn id(&self) -> Option<ObjectId> {
        self.id
    }
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn class(&self) -> &Class {
        &self.class
    }
}