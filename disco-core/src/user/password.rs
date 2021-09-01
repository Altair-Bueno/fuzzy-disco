use serde::{Serialize,Deserialize};
use validator::Validate;
use crate::user::result::UserError;
use crate::user::result;
use bcrypt::DEFAULT_COST;



#[derive(Serialize,Deserialize, Debug,Validate,Ord, PartialOrd, PartialEq,Eq)]
pub struct Password {
    // Check for non empty hashed string
    #[validate(length(min = 1))]
    password:String
}


impl Password {
    pub fn new(s:&str) -> result::Result<Password>{
        if s.len() < 8  {
            Err(UserError::InvalidPassword)
        } else {
            let hashed_password =bcrypt::hash(s,DEFAULT_COST);
            match hashed_password {
                Ok(password) => Ok(Password { password }),
                Err(_) => Err(UserError::HashPassword)
            }
        }
    }
}