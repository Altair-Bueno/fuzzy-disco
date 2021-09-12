use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::mongo::media::result::MediaError;


/// Different media formats that can be stored on the server. This includes
/// images and audio files from different file formats
///
/// # Supported list
///
/// ## Image
/// - jpeg
/// - png
/// - gif
///
/// ## Audio
/// - mpeg
/// - m4a
/// - wav
/// - aac
#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
pub enum Format {
    Audio,
    Image,
}

impl FromStr for Format {
    type Err = MediaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = match s {
            "image/jpeg" | "image/png" | "image/gif" => Format::Image,
            "audio/mpeg"| "audio/m4a" | "audio/x-wav" | "audio/aac" => Format::Audio,
            format => return Err(MediaError::InvalidFormat(format.to_string())),
        };
        Ok(r)
    }
}
