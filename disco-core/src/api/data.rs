use serde::{Deserialize,Serialize};
use crate::mongo::user::User;
use crate::mongo::post::Post;
use crate::mongo::visibility::Visibility;

#[derive(Debug,Serialize,Deserialize)]
pub struct ApiUserResponse {
    pub alias: String,
    pub description: Option<String>,
    pub avatar: Option<String>
}

impl From<User> for ApiUserResponse {
    fn from(u: User) -> Self {
        ApiUserResponse {
            alias: u.alias().to_string(),
            description: u.description().clone().map(|x| x.to_string()),
            avatar: u.avatar().map(|x| x.to_string())
        }
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ApiPostResponse {
    id: Option<String>,
    title: String,
    caption: String,
    author: String,
    audio: String,
    photo: String,
    visibility: Visibility,
    creation_date: String
}

impl From<Post> for ApiPostResponse {
    fn from(p: Post) -> Self {
        ApiPostResponse {
            id: p.id().map(|x| x.to_string()),
            title: p.title().to_string(),
            caption: p.caption().to_string(),
            author: p.author().to_string(),
            audio: p.audio().to_string(),
            photo: p.photo().to_string(),
            visibility: p.visibility().clone(),
            creation_date: p.creation_date().to_string()
        }
    }
}