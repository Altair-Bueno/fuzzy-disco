use serde::Serialize;
use serde::Deserialize;


pub type Result<E> = std::result::Result<E,PostError>;

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum PostError {
    InvalidTitle,
    InvalidCaption,
}