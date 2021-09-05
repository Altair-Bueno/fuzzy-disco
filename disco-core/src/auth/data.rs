use serde::{Deserialize, Serialize};

use crate::mongo::user::{User, UserError};
use crate::mongo::IntoDocument;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSingUp<'a> {
    alias: &'a str,
    email: &'a str,
    password: &'a str,
}

impl IntoDocument<User> for UserSingUp<'_> {
    type Err = UserError;

    fn validate(self) -> Result<User, Self::Err> {
        let UserSingUp {
            alias,
            email,
            password,
        } = self;
        let alias = alias.parse()?;
        let email = email.parse()?;
        let password = password.parse()?;
        Ok(User::new(alias, email, password))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogInEmail<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogInAlias<'a> {
    pub alias: &'a str,
    pub password: &'a str,
}
