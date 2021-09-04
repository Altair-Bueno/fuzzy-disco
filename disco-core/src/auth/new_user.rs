use serde::{Deserialize, Serialize};

use crate::mongo::traits::{Document, IntoDocument};
use crate::mongo::user::{Alias, Email, Password, User, UserError};

#[derive(Debug,Serialize,Deserialize)]
pub struct NewUser<'a> {
    alias:&'a str,
    email:&'a str,
    password:&'a str,
}

impl IntoDocument<User> for NewUser<'_>  {
    type Err = UserError;

    fn validate(self: Self) -> Result<User, Self::Err> {
        let NewUser { alias, email, password } = self;
        let alias = alias.parse()?;
        let email = email.parse()?;
        let password = password.parse()?;
        Ok(User::new(alias, email, password))
    }
}