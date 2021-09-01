use std::str::FromStr;
use serde::Deserialize;
use serde::Serialize;
use crate::post::result;
use rocket::serde::de::Error;
use rocket::serde::Deserializer;
use validator::Validate;


#[derive(Ord, PartialOrd, PartialEq,Eq, Debug, Serialize, Deserialize)]
pub struct Title {
    title: String
}

impl FromStr for Title {
    type Err = crate::post::result::PostError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Title::new(s)
    }
}

impl Title {
    pub fn new(s:&str) -> result::Result<Title> {
        let trim = s.trim();
        if trim.is_empty() {
            Err(crate::post::result::PostError::InvalidTitle)
        } else {
            Ok(Title{
                title: trim.to_string()
            })
        }
    }
    pub fn get_title(&self) -> &str {
        self.title.as_str()
    }
}

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
            (" hello world    ", "hello world"),
            ("favourite music", "favourite music"),
            ("Valid char&", "Valid char&")
        ];
        for (s, expect) in list {
            let title = Title::new(s).unwrap();
            assert_eq!(title.get_title(), expect)
        }
    }
}