pub use alias::Alias;
pub use password::Password;
pub use user::User;
pub use result::UserError;
pub use result::Result;
pub use email::Email;

mod alias;
mod password;
pub mod result;
#[allow(dead_code)]
mod user;
mod email;
