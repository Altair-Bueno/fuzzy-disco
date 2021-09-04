use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use validator::{Validate, ValidationError};

use crate::mongo::post::result;
use crate::mongo::post::result::PostError;

lazy_static! {
    /// Valid title must match r"^(\S+.*\S)$"
    static ref RE: Regex = Regex::new(r"^(\S+.*\S)$").unwrap();
}
/// Max title legth
const MAX_TITLE_LENGTH: usize = 24;

/// A title represents a non empty string of text that is trimmed and matches the
/// [RE] regex with legth <= [MAX_TITLE_LENGTH]
#[derive(Validate, Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Title {
    #[validate(custom = "validate_title")]
    title: String,
}

fn validate_title(s: &str) -> Result<(), ValidationError> {
    let reg = RE.is_match(s);
    if reg && s.len() <= MAX_TITLE_LENGTH {
        Ok(())
    } else if reg {
        Err(ValidationError::new("Title too small"))
    } else {
        Err(ValidationError::new("Title is not trimmed"))
    }
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
        match validate_title(s) {
            Ok(_) => Ok(Title {
                title: s.to_string(),
            }),
            Err(_) => Err(PostError::InvalidTitle),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::mongo::post::title::Title;

    #[test]
    pub fn empty() {
        let strings = vec![" ", "", "      ", "       "];
        for s in strings {
            let title = Title::new(s);
            assert!(matches!(title, Err(_)))
        }
    }

    #[test]
    pub fn non_empty() {
        let list = vec![" hello world    ", "        world", "         world     "];
        for s in list {
            let title = Title::new(s);
            assert!(matches!(title, Err(_)))
        }
    }

    #[test]
    pub fn long() {
        let string = "iksadfjlisfdajkhlsdfafsdhjkfdsjkhfdsajkhfsdahjkfdsahjkfdasfdsjkhadfsajkhldfasjkhdfsajkhsdfakhjldfsajkhfdsahjkdfsa";
        assert!(matches!(string.parse::<Title>(), Err(_)))
    }
}
