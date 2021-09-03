use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, PartialEq, Eq, Default)]
pub struct PostPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_path: Option<String>,
}

impl PostPayload {
    pub fn new() -> PostPayload {
        Default::default()
    }
    pub fn id(&self) -> &Option<String> {
        &self.id
    }
    pub fn title(&self) -> &Option<String> {
        &self.title
    }
    pub fn caption(&self) -> &Option<String> {
        &self.caption
    }
    pub fn author_id(&self) -> &Option<String> {
        &self.author_id
    }
    pub fn audio_path(&self) -> &Option<String> {
        &self.audio_path
    }
    pub fn photo_path(&self) -> &Option<String> {
        &self.photo_path
    }
    pub fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }
    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }
    pub fn set_caption(&mut self, caption: Option<String>) {
        self.caption = caption;
    }
    pub fn set_author_id(&mut self, author_id: Option<String>) {
        self.author_id = author_id;
    }
    pub fn set_audio_path(&mut self, audio_path: Option<String>) {
        self.audio_path = audio_path;
    }
    pub fn set_photo_path(&mut self, photo_path: Option<String>) {
        self.photo_path = photo_path;
    }
}

#[cfg(test)]
mod test {
    use crate::api::posts_payload::PostPayload;

    #[test]
    pub fn test() {
        let json = "{  \"id\": \"278348jfwduhq32r\",  \"title\": \"Hello world\",  \"caption\": \"The caption for the text\",  \"author_id\": \"u8ssdafjjk23uh4ro\",  \"audio_path\": \"/route\",  \"photo_path\": \"/route/photo\"}";
        let _: PostPayload = serde_json::from_str(json).unwrap();
    }
}
