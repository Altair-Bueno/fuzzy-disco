use serde::Deserialize;
use serde::Serialize;

pub type Result<E> = std::result::Result<E, UserError>;

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum UserError {
    PasswordTooShort,
    HashError,
    InvalidUsername,
    InvalidEmail,
}
