use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::mongo::media::format::Format;
use crate::mongo::traits::Document;
use crate::mongo::media::Status;

/// A Media instance represents a Document on Mongodb with usefull information
/// related to multimedia files
#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Media {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    status: Status,
    format: Format,
}

impl Document for Media {}

impl Media {
    pub fn new(class: Format) -> Media {
        Media {
            id: None,
            status: Status::Waiting,
            format: class,
        }
    }

    pub fn id(&self) -> Option<ObjectId> {
        self.id
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
    pub fn format(&self) -> Format {
        self.format
    }
}
