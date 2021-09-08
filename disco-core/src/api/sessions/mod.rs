use mongodb::bson::doc;
use mongodb::Collection;
use rocket::State;

use crate::api::result::ApiError;
use crate::mongo::session::session;
use crate::mongo::user::{Alias, User};

pub mod data;
pub mod get;
pub mod post;

pub async fn delete_all_sessions_from(
    user_alias: &Alias,
    session_collection: &State<Collection<session>>,
) -> Result<(), ApiError> {
    let filter = doc! { "user_alias": user_alias.to_string() };
    session_collection
        .delete_many(filter, None)
        .await
        .map(|_| ())
        .map_err(|x| ApiError::DatabaseError(x))
}
