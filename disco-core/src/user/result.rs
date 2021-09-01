use serde::Serialize;
use serde::Deserialize;


pub type Result<E> = std::result::Result<E,UserError>;

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum UserError {
    InvalidPassword,
    HashPassword,
    InvalidUsername
}
