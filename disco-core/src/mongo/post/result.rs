use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

pub type Result<E> = std::result::Result<E, PostError>;

/// Errors produced while creating a Post document
#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize,Error)]
pub enum PostError {
    /// The given title does not match the expected requirements
    #[error("Invalid title format")]
    InvalidTitleFormat,
    /// The given caption does not match the expected requirements
    #[error("Caption must be < {0} characters long")]
    CaptionTooLong(usize),
    /// The given path is not a valid URI
    #[error("The given string is not a valid URI")]
    InvalidURI,
}
