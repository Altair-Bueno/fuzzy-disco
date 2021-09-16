use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use rocket::request::FromParam;
use serde::{Deserialize, Serialize};

use crate::mongo::user::result;
use crate::mongo::user::result::UserError;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^[a-zA-Z_\-0-9]{4,30}$").unwrap();
}

/// An alias represents the User's custom username for his account. For an alias
/// to be valid, it must mach r"^[a-zA-Z_\-0-9]{4,30}$"
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
#[serde(transparent)]
pub struct Alias {
    alias: String,
}

impl<'a> FromParam<'a> for Alias {
    type Error = UserError;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        FromStr::from_str(param)
    }
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
    /// Creates a new alias instance if possible
    pub fn new(s: &str) -> result::Result<Alias> {
        if RE.is_match(s) {
            Ok(Alias {
                alias: s.to_string(),
            })
        } else {
            Err(UserError::InvalidUsername)
        }
    }

    pub fn alias(&self) -> &str {
        &self.alias
    }
}

impl From<Alias> for mongodb::bson::Bson {
    fn from(a: Alias) -> Self {
        mongodb::bson::to_bson(&a).unwrap()
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
    fn invalid() {
        let username = "Hello world";
        assert!(matches!(username.parse::<Alias>(), Err(_)))
    }
}
