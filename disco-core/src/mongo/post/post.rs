use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::mongo::post::caption::Caption;
use crate::mongo::post::title::Title;

#[derive(Serialize, Deserialize, Debug, Validate, Ord, PartialOrd, PartialEq, Eq)]
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
    pub fn new(
        title: Title,
        caption: Caption,
        author_id: ObjectId,
        audio_path: String,
        photo_path: String,
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
            "/path".to_string(),
            "/path".to_string(),
        );
        let ser = serde_json::to_string(&post).unwrap();
        let des: Post = serde_json::from_str(ser.as_str()).unwrap();

        assert_eq!(des, post)
    }

    #[test]
    pub fn deserialize() {
        let json = "{\"_id\":{\"$oid\":\"612fc6d4b57f3339cf097434\"},\"title\":\"Hello world\",\"caption\":\"Caption text\",\"author_id\":{\"$oid\":\"612fc6d4b57f3339cf097434\"},\"audio_path\":\"/path/to/file\",\"photo_path\":\"path/to/photo\"}";
        let _:Post = serde_json::from_str(json).unwrap();
    }
}
