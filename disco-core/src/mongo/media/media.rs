use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::mongo::media::format::Format;
use crate::mongo::media::Status;
use crate::mongo::traits::Document;
use crate::mongo::user::Alias;
use crate::mongo::visibility::Visibility;

/// A Media instance represents a Document on Mongodb with usefull information
/// related to multimedia files
#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Media {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    uploaded_by: Alias,
    status: Status,
    format: Format,
    visibility: Visibility,
}

impl Document for Media {}

impl Media {
    pub fn new(alias: Alias, class: Format) -> Media {
        Media {
            id: None,
            uploaded_by: alias,
            status: Status::Waiting,
            format: class,
            visibility: Visibility::Private,
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
    pub fn uploaded_by(&self) -> &Alias {
        &self.uploaded_by
    }
    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }
}
