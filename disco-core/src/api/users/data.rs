use serde::{Serialize,Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePassword<'a> {
    pub new_password: Option<&'a str>,
    pub password: &'a str
}