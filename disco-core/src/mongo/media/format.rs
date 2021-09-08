use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::mongo::media::result::MediaError;

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
pub enum Format {
    #[serde(rename = "png")]
    PNG,
    #[serde(rename = "jpeg")]
    JPEG,
    #[serde(rename = "mp3")]
    MP3,
}

impl FromStr for Format {
    type Err = MediaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = match s {
            "image/jpeg" => Format::JPEG,
            "image/png" => Format::PNG,
            "audio/mpeg" => Format::MP3,
            format => return Err(MediaError::InvalidFormat(format.to_string()))
        };
        Ok(r)
    }
}
