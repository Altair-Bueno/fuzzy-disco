use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::mongo::post::caption::Caption;
use crate::mongo::post::title::Title;
use crate::mongo::traits::Document;
use crate::mongo::user::Alias;

/// Represents a stored document on a document based database such as MongoDB.
/// Althought JSON does not enforce any kind of schema, Rust type safety allows
/// us to enforce certain rules
///
/// # Valid document
///
/// A Post document is considered to be **valid** when all of his childs are
/// valid too. For more information check their childs
/// - [mongodb::bson::oid::ObjectId]
/// - [crate::mongo::post::Title]
/// - [crate::mongo::post::Caption]
#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct Post {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    title: Title,
    caption: Caption,
    author: Alias,
    audio: ObjectId,
    photo: ObjectId,
}

impl Document for Post {}

impl Post {
    /// Creates a new post instance with the recived arguments
    pub fn new(
        title: Title,
        caption: Caption,
        author: Alias,
        audio: ObjectId,
        photo: ObjectId,
    ) -> Self {
        Post {
            id: None,
            title,
            caption,
            author,
            audio,
            photo,
        }
    }


    pub fn id(&self) -> Option<ObjectId> {
        self.id
    }
    pub fn title(&self) -> &Title {
        &self.title
    }
    pub fn caption(&self) -> &Caption {
        &self.caption
    }
    pub fn author(&self) -> &Alias {
        &self.author
    }
    pub fn audio(&self) -> ObjectId {
        self.audio
    }
    pub fn photo(&self) -> ObjectId {
        self.photo
    }
}
