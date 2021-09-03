use bcrypt::DEFAULT_COST;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::mongo::user::result;
use crate::mongo::user::result::UserError;

#[derive(Serialize, Deserialize, Debug, Validate, Ord, PartialOrd, PartialEq, Eq)]
pub struct Password {
    // Check for non empty hashed string
    #[validate(length(min = 1))]
    password: String,
}
impl ToString for Password {
    fn to_string(&self) -> String {
        self.password.to_string()
    }
}

impl Password {
    pub fn new(s: &str) -> result::Result<Password> {
        if s.len() < 8 {
            Err(UserError::InvalidPassword)
        } else {
            let hashed_password = bcrypt::hash(s, DEFAULT_COST);
            match hashed_password {
                Ok(password) => Ok(Password { password }),
                Err(_) => Err(UserError::HashPassword),
            }
        }
    }
}
