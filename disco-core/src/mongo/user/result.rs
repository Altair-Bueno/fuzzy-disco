use serde::Deserialize;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use thiserror::Error;

pub type Result<E> = std::result::Result<E, UserError>;

/// Different errors related to user fields
#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize,Error)]
pub enum UserError {
    /// Passwords must have at least 8 characters long
    #[error("Password must be {0} characters long")]
    PasswordTooShort(usize),
    /// Couldn't hash the given password
    #[error("Couldn't hash the given password")]
    HashError,
    /// Username doesn't meet the requirements
    #[error("Invalid username")]
    InvalidUsername,
    /// Email doesn't meed the requirements
    #[error("Invalid email")]
    InvalidEmail,
}