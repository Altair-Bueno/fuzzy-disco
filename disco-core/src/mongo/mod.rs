pub use traits::IntoDocument;

/// Contains data structures that represents media files on a document-based
/// database
pub mod media;
/// Contains data structures that represents users' posts on a document-based
/// database
pub mod post;
/// Contains data structures that represents a user session
#[allow(dead_code)]
pub mod session;
/// Traits related to Mongo documents
mod traits;
/// Contains data structures that represents users on a document-based database
pub mod user;
/// Enum for profile, post and media visibility
pub mod visibility;
