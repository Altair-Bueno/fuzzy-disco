use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub enum AuthError {
    DecodeError,
    EncodeError,
    ExpiredToken,
}

pub type AuthResult<T> = Result<T, AuthError>;
