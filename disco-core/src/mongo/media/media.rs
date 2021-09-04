use serde::{Deserialize, Serialize};
use validator::Validate;
use mongodb::bson::oid::ObjectId;
use crate::mongo::media::format::Format;

// TODO better doc
/// A Media instance contains information about how to locate a resource on the
/// server as an absolute path.
#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize,Validate)]
pub struct Media {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    url: String,
    class: Format,
}

impl Media {
    pub fn new(url:&str, class: Format) -> Media {
        Media {
            id: None,
            url:url.to_string(),
            class
        }
    }

    pub fn id(&self) -> Option<ObjectId> {
        self.id
    }
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn class(&self) -> &Format {
        &self.class
    }
}