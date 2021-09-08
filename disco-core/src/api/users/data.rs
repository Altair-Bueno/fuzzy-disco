use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePassword<'a> {
    pub new_password: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser<'a> {
    pub email: Option<&'a str>,
}
