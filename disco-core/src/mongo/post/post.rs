use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::mongo::post::caption::Caption;
use crate::mongo::post::media::Media;
use crate::mongo::post::title::Title;

/// Represents a stored document on a document based database such as MongoDB.
/// Althought JSON does not enforce any kind of schema, Rust type safety allows
/// us to enforce certain rules
///
/// # Valid document
///
/// A Post document is considered to be **valid** when all of his childs are
/// valid too. For more information check their childs
/// - [mongodb::bson::oid::ObjectId]
/// - [crate::mongo::post::title::Title]
/// - [crate::mongo::post::caption::Caption]
#[derive(Serialize, Deserialize, Debug, Validate, Ord, PartialOrd, PartialEq, Eq)]
pub struct Post {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,

    #[validate]
    title: Title,

    #[validate]
    caption: Caption,
    author_id: ObjectId,
    #[validate]
    audio_path: Media,
    #[validate]
    photo_path: Media,
}

impl Post {
    /// Creates a new post instance with the recived arguments
    pub fn new(
        title: Title,
        caption: Caption,
        author_id: ObjectId,
        audio_path: Media,
        photo_path: Media,
    ) -> Self {
        Post {
            id: None,
            title,
            caption,
            author_id,
            audio_path,
            photo_path,
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
    pub fn author_id(&self) -> ObjectId {
        self.author_id
    }
    pub fn audio_path(&self) -> &Media {
        &self.audio_path
    }
    pub fn photo_path(&self) -> &Media {
        &self.photo_path
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
    pub fn set_author_id(&mut self, author_id: ObjectId) {
        self.author_id = author_id;
    }
    pub fn set_audio_path(&mut self, audio_path: Media) {
        self.audio_path = audio_path;
    }
    pub fn set_photo_path(&mut self, photo_path: Media) {
        self.photo_path = photo_path;
    }
}

#[cfg(test)]
mod test {
    use crate::mongo::post::post::Post;

    #[test]
    pub fn serialize() {
        let post = Post::new(
            "My post".parse().unwrap(),
            "Amazing post".parse().unwrap(),
            mongodb::bson::oid::ObjectId::new(),
            "/path".parse().unwrap(),
            "/path".parse().unwrap(),
        );
        let ser = serde_json::to_string(&post).unwrap();
        let des: Post = serde_json::from_str(ser.as_str()).unwrap();

        assert_eq!(des, post)
    }

    #[test]
    pub fn deserialize() {
        let json = "{\"_id\":{\"$oid\":\"612fc6d4b57f3339cf097434\"},\"title\":\"Hello world\",\"caption\":\"Caption text\",\"author_id\":{\"$oid\":\"612fc6d4b57f3339cf097434\"},\"audio_path\":\"/path/to/file\",\"photo_path\":\"path/to/photo\"}";
        let _: Post = serde_json::from_str(json).unwrap();
    }
}
