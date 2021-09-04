use serde::{Serialize,Deserialize};
#[derive (Serialize,Deserialize,Eq, PartialEq,Ord, PartialOrd,Debug)]
pub enum Class {
    #[serde(rename = "image/png")]
    PNG,
    #[serde(rename = "jpeg")]
    JPEG,
    #[serde(rename = "mp3")]
    MP3,
}

