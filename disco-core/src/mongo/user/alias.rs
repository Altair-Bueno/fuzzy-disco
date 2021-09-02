use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::mongo::user::result;
use crate::mongo::user::result::UserError;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^[a-zA-Z_\-0-9]{4,30}$").unwrap();
}

#[derive(Serialize, Deserialize, Debug, Validate, Eq, PartialEq, Ord, PartialOrd)]
pub struct Alias {
    #[validate(regex = "RE")]
    alias: String,
}
impl ToString for Alias {
    fn to_string(&self) -> String {
        self.alias.to_string()
    }
}
impl FromStr for Alias {
    type Err = crate::mongo::user::result::UserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Alias::new(s)
    }
}

impl Alias {
    pub fn new(s: &str) -> result::Result<Alias> {
        if RE.is_match(s) {
            Ok(Alias {
                alias: s.to_string(),
            })
        } else {
            Err(UserError::InvalidUsername)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::mongo::user::alias::Alias;

    #[test]
    fn valid() {
        let username = "Altair-Bueno";
        let _: Alias = username.parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid() {
        let username = "Hello world";
        let _: Alias = username.parse().unwrap();
    }
}
