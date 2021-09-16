use serde::{Deserialize, Serialize};
use crate::api::data::ObjectIdWrapper;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePassword<'a> {
    pub new_password: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser<'a> {
    pub email: Option<&'a str>,
    pub description: Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarPictureID {
    pub media_id: Option<ObjectIdWrapper>,
}
