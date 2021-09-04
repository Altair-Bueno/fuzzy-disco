use serde::Deserialize;
use serde::Serialize;

pub type Result<E> = std::result::Result<E, PostError>;

/// Errors produced while creating a Post document
#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum PostError {
    /// The given title does not match the expected requirements
    InvalidTitleFormat,
    /// The given caption does not match the expected requirements
    CaptionTooLong,
    /// The given path is not a valid URI
    InvalidURI,
}
