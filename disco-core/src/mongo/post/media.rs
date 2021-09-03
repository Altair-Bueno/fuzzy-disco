use crate::mongo::post::result::PostError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;
use validator::Validate;

/// A Media instance contains information about how to locate a resource

// TODO validate that the URI is valid, or check if we can provide a mediaID
#[derive(Validate, Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Media {
    //#[validate(url)]
    uri: String,
}

impl ToString for Media {
    fn to_string(&self) -> String {
        self.uri.to_string()
    }
}
impl FromStr for Media {
    type Err = PostError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Media::new(s)
    }
}

impl Media {
    pub fn new(s: &str) -> crate::mongo::post::result::Result<Media> {
        Ok(Media { uri: s.to_string() })
    }
}

#[cfg(test)]
mod test {
    use crate::mongo::post::media::Media;

    #[test]
    pub fn valid() {
        let a = "https://www.edix.com/es/instituto/videos-gratis/";
        let _: Media = a.parse().unwrap();
    }
    #[test]
    pub fn valid2() {
        let a = "/hello";
        let _: Media = a.parse().unwrap();
    }
    #[test]
    #[should_panic]
    #[ignore]
    pub fn invalid() {
        let a = "Hello world";
        let _: Media = a.parse().unwrap();
    }
}
