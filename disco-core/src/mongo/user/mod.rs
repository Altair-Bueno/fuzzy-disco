pub use alias::Alias;
pub use email::Email;
pub use password::Password;
pub use result::Result;
pub use result::UserError;
pub use user::User;

pub use crate::mongo::sesion::Sesion;

mod alias;
mod email;
mod password;
pub mod result;
#[allow(dead_code)]
mod user;
