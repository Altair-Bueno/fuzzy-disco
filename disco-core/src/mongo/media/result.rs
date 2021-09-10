use thiserror::Error;

#[derive(Error, Debug)]
pub enum MediaError {
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
}
