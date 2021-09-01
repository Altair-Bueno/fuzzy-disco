use serde::{Serialize,Deserialize};
use crate::post::title::Title;

#[derive(Serialize,Deserialize)]
pub struct Post {
    title: Title,
    caption: String,
    author_id: String,
    audio_path: String,
    photo_path: String,
}

impl Post {
    pub fn new(title: Title, caption: String, author_id: String, audio_path: String, photo_path: String) -> Self {
        Post { title, caption, author_id, audio_path, photo_path }
    }

    pub fn title(&self) -> &Title {
        &self.title
    }
    pub fn caption(&self) -> &str {
        &self.caption
    }
    pub fn author_id(&self) -> &str {
        &self.author_id
    }
    pub fn audio_path(&self) -> &str {
        &self.audio_path
    }
    pub fn photo_path(&self) -> &str {
        &self.photo_path
    }
}