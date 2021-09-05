use std::str::FromStr;

use bcrypt::{DEFAULT_COST, Version};
use serde::{Deserialize, Serialize};

use crate::mongo::user::result;
use crate::mongo::user::result::UserError;

const SALT : &[u8]= b"SALTSAFDF2SALTSA";

/// A Password instance represents a [bcrypt] encripted hash that is stored on
/// the database. The hash is used to autheticate the user without storing the
/// real password
#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, PartialEq, Eq)]
#[serde(transparent)]
pub struct Password {
    password: String,
}

impl ToString for Password {
    fn to_string(&self) -> String {
        self.password.to_string()
    }
}

impl FromStr for Password {
    type Err = UserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Password::new(s)
    }
}

impl Password {
    /// Creates a new hashed password from a text string. All strings used as
    /// passwords should be at least 8 characters long
    pub fn new(s: &str) -> result::Result<Password> {
        if s.len() < 8 {
            Err(UserError::PasswordTooShort)
        } else {
            let hashed_password = bcrypt::hash_with_salt(s,DEFAULT_COST,SALT)
                .map(|x| x.format_for_version(Version::TwoB));
            match hashed_password {
                Ok(password) => Ok(Password { password }),
                Err(_) => Err(UserError::HashError),
            }
        }
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}
