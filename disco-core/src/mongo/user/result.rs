use serde::Deserialize;
use serde::Serialize;

pub type Result<E> = std::result::Result<E, UserError>;

/// Different errors related to user fields
#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum UserError {
    /// Passwords must have at least 8 characters long
    PasswordTooShort,
    /// Couldn't hash the given password
    HashError,
    /// Username doesn't meet the requirements
    InvalidUsername,
    /// Email doesn't meed the requirements
    InvalidEmail,
}
