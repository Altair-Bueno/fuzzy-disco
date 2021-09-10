use mongodb::bson::doc;
use mongodb::Collection;
use rocket::State;

use crate::api::result::ApiError;
use crate::mongo::session::Session;
use crate::mongo::user::Alias;
use crate::api::SESSION_USER_ALIAS;

mod data;
pub mod get;
pub mod post;

pub async fn delete_all_sessions_from(
    user_alias: &Alias,
    session_collection: &State<Collection<Session>>,
) -> Result<(), ApiError> {
    let filter = doc! { SESSION_USER_ALIAS: mongodb::bson::to_bson(user_alias).unwrap() };
    session_collection
        .delete_many(filter, None)
        .await
        .map(|_| ())
        .map_err(ApiError::DatabaseError)
}
