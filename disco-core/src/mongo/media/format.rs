use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::mongo::media::result::MediaError;

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
pub enum Format {
    Audio,
    Image
}

impl FromStr for Format {
    type Err = MediaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = match s {
            "image/jpeg" => Format::Image,
            "image/png" => Format::Image,
            "audio/mpeg" => Format::Audio,
            format => return Err(MediaError::InvalidFormat(format.to_string()))
        };
        Ok(r)
    }
}
