use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::mongo::user::result::UserError;

/// Represents a valid email address
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[serde(transparent)]
pub struct Email {
    email: String,
}

impl FromStr for Email {
    type Err = UserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Email::new(s)
    }
}

impl Email {
    pub fn new(s: &str) -> crate::mongo::user::result::Result<Email> {
        if validator::validate_email(s) {
            Ok(Email {
                email: s.to_string(),
            })
        } else {
            Err(UserError::InvalidEmail)
        }
    }

    pub fn email(&self) -> &str {
        &self.email
    }
}

#[cfg(test)]
mod test {
    use super::Email;

    #[test]
    pub fn valid() {
        for v in vec![
            "hello@gmail.com",
            "discord33@outlook.com",
            "example@company.org",
        ] {
            assert!(matches!(v.parse::<Email>(), Ok(_)))
        }
    }

    #[test]
    pub fn invalid() {
        let list = vec!["", " ", "@com", "pepe", "exampl @hello.com"];
        for e in list {
            assert!(matches!(e.parse::<Email>(), Err(_)))
        }
    }
}
