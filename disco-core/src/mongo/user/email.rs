use serde::{Deserialize,Serialize};
use validator::Validate;
use std::str::FromStr;
use crate::mongo::user::result::UserError;

#[derive(Serialize, Deserialize, Debug, Validate, Eq, PartialEq, Ord, PartialOrd)]
#[serde(transparent)]
pub struct Email {
    #[validate(email)]
    email:String,
}

impl FromStr for Email {
    type Err = UserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Email::new(s)
    }
}

impl Email {
    pub fn new(s:&str) -> crate::mongo::user::result::Result<Email> {
        if validator::validate_email(s) {
            Ok(Email{ email: s.to_string() })
        } else {
            Err(UserError::InvalidEmail)
        }
    }
}