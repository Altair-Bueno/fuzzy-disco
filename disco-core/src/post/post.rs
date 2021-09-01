use serde::{Serialize,Deserialize};
use crate::post::title::Title;
use crate::post::caption::Caption;
use validator::Validate;
use mongodb::bson::oid::ObjectId;
use std::path::Path;

#[derive(Serialize,Deserialize, Debug,Validate)]
pub struct Post {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[validate]
    #[serde(flatten)]
    title: Title,
    #[validate]
    #[serde(flatten)]
    caption: Caption,
    author_id: ObjectId,
    audio_path: String,
    photo_path: String,
}

impl Post {
    pub fn new(title: Title, caption: Caption, author_id: ObjectId, audio_path: String, photo_path: String) -> Self {
        Post { id: None, title, caption, author_id , audio_path, photo_path }
    }
}