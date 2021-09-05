/// Contains data structures that represents media files on a document-based
/// database
pub mod media;
/// Contains data structures that represents users' posts on a document-based
/// database
pub mod post;
mod traits;
/// Contains data structures that represents users on a document-based database
pub mod user;

pub use traits::IntoDocument;