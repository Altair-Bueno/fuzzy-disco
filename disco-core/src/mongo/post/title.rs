use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

use crate::mongo::post::result;
use crate::mongo::post::result::PostError;

/// Max title legth
const MAX_TITLE_LENGTH: usize = 40;

/// A title represents a non empty string of text whose length is
/// <= [MAX_TITLE_LENGTH]
#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct Title {
    title: String,
}

impl ToString for Title {
    fn to_string(&self) -> String {
        self.title.to_string()
    }
}

impl FromStr for Title {
    type Err = crate::mongo::post::result::PostError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Title::new(s)
    }
}

impl Title {
    /// Creates a new title, if possible
    pub fn new(s: &str) -> result::Result<Title> {
        if s.len() <= MAX_TITLE_LENGTH {
            Ok(Title {
                title: s.to_string(),
            })
        } else {
            Err(PostError::InvalidTitleFormat)
        }
    }
}