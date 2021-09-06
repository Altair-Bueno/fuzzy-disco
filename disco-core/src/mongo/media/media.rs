use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::mongo::media::format::Format;
use crate::mongo::traits::Document;

/// A Media instance represents a Document on Mongodb with usefull information
/// related to multimedia files
#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Media {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    url: String,
    format: Format,
}

impl Document for Media {}

impl Media {
    pub fn new(url: &str, class: Format) -> Media {
        Media {
            id: None,
            url: url.to_string(),
            format: class,
        }
    }

    pub fn id(&self) -> Option<ObjectId> {
        self.id
    }
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn class(&self) -> &Format {
        &self.format
    }
}
