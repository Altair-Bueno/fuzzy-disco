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
#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, PartialEq, Eq,Clone)]
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
    pub fn audio_path(&self) -> &ObjectId {
        &self.audio
    }
    pub fn photo_path(&self) -> &ObjectId {
        &self.photo
    }
    pub fn set_id(&mut self, id: Option<ObjectId>) {
        self.id = id;
    }
    pub fn set_title(&mut self, title: Title) {
        self.title = title;
    }
    pub fn set_caption(&mut self, caption: Caption) {
        self.caption = caption;
    }
    pub fn set_author_id(&mut self, author_id: Alias) {
        self.author = author_id;
    }
    pub fn set_audio_path(&mut self, audio_path: ObjectId) {
        self.audio = audio_path;
    }
    pub fn set_photo_path(&mut self, photo_path: ObjectId) {
        self.photo = photo_path;
    }
}
