use serde::{Deserialize, Serialize};

use crate::mongo::visibility::Visibility;

#[derive(Debug, Serialize, Deserialize)]
pub struct EditPostPayload {
    visibility: Visibility,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPostPayload<'a> {
    pub(crate) title: &'a str,
    pub(crate) caption: &'a str,
    pub(crate) audio: &'a str,
    pub(crate) photo: &'a str,
    pub(crate) visibility: &'a str,
}
