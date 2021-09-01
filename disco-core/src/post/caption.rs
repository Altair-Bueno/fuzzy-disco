use serde::Deserialize;
use serde::Serialize;
use validator::Validate;
use std::str::FromStr;
use crate::post::result::PostError;

const MAX_LENGTH_CAPTION: usize = 150;

#[derive(Serialize,Deserialize,Validate,Debug,Eq, PartialEq,Ord, PartialOrd)]
pub struct Caption {
    #[validate(length (max = "MAX_LENGTH_CAPTION"))]
    caption: String
}

impl FromStr for Caption {
    type Err = crate::post::result::PostError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Caption::new(s)
    }
}

impl Caption {
    pub fn new (s:&str) -> crate::post::result::Result<Caption>{
        if s.len() > MAX_LENGTH_CAPTION {
            Err(PostError::InvalidCaption)
        } else {
            Ok(Caption{ caption: s.to_string() })
        }
    }

    pub fn caption(&self) -> &str {
        self.caption.as_str()
    }
}

#[cfg(test)]
mod test {
    use crate::post::caption::Caption;
    #[test]
    pub fn allowed() {
        let caption = "jhsdfahjsfdjh";
        let _: Caption = caption.parse().unwrap();
    }

    #[test]
    #[should_panic]
    pub fn not_allowed() {
        let caption = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let _ :Caption = caption.parse().unwrap();
    }
}