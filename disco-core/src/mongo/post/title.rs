use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

use crate::mongo::post::result;
use crate::mongo::post::result::PostError;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\S+.*\S)|\S$").unwrap();
}
/// Max title legth
const MAX_TITLE_LENGTH: usize = 40;

/// A title represents a non empty string of text that is trimmed and matches the
/// r"^(\S+.*\S)|\S$" regex with length <= [MAX_TITLE_LENGTH]
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
        if RE.is_match(s) && s.len() <= MAX_TITLE_LENGTH {
            Ok(Title {
                title: s.to_string(),
            })
        } else {
            Err(PostError::InvalidTitleFormat)
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
