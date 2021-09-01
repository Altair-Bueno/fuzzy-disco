use std::str::FromStr;
use serde::Deserialize;
use serde::Serialize;
use crate::post::result;
use validator::{Validate, ValidationError};
use regex::Regex;
use crate::post::result::PostError;
use lazy_static::lazy_static;

lazy_static!{
    static ref RE: Regex = Regex::new(r"^(\S+.*\S)$").unwrap();
}
const MAX_TITLE_LENGTH: usize= 24;

#[derive(Validate, Ord, PartialOrd, PartialEq,Eq, Debug, Serialize, Deserialize)]
pub struct Title {
    #[validate(custom = "validate_title")]
    title: String
}

fn validate_title (s : &str)-> Result<(),ValidationError> {
    let reg = RE.is_match(s);
    if reg && s.len() <= MAX_TITLE_LENGTH {
        Ok(())
    } else if reg {
        Err(ValidationError::new("Title too small"))
    } else {
        Err(ValidationError::new("Title is not trimmed"))
    }
}

impl FromStr for Title {
    type Err = crate::post::result::PostError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Title::new(s)
    }
}

impl Title {
    pub fn new(s:&str) -> result::Result<Title> {
        match validate_title(s) {
            Ok(_) => Ok(Title { title: s.to_string()  }),
            Err(_) => Err(PostError::InvalidTitle)
        }
    }
    pub fn get_title(&self) -> &str {
        self.title.as_str()
    }
}

#[cfg(test)]
mod test {
    use crate::post::title::Title;

    #[test]
    pub fn empty() {
        let strings = vec![
            " ",
            "",
            "      ",
            "       ",
        ];
        for s in strings {
            let title = Title::new(s);
            assert!(matches!(title,Err(_)))
        }
    }

    #[test]
    pub fn non_empty() {
        let list = vec![
            " hello world    ",
            "        world",
            "         world     "
        ];
        for s in list {
            let title = Title::new(s);
            assert!(matches!(title,Err(_)))
        }
    }

    #[test]
    #[should_panic]
    pub fn long() {
        let string = "iksadfjlisfdajkhlsdfafsdhjkfdsjkhfdsajkhfsdahjkfdsahjkfdasfdsjkhadfsajkhldfasjkhdfsajkhsdfakhjldfsajkhfdsahjkdfsa";
        let title = Title::new(string).unwrap();
    }
}