use mongodb::bson::doc;
use mongodb::Collection;
use rocket::State;

use crate::api::result::{ApiError, ApiResult};
use crate::api::SESSION_USER_ALIAS;
use crate::mongo::session::Session;
use crate::mongo::user::Alias;

/// Data Structures used on this module
mod data;
/// GET /api/sessions/
pub mod get;
/// POST /api/sessions/
pub mod post;

pub async fn delete_all_sessions_from(
    user_alias: &Alias,
    session_collection: &State<Collection<Session>>,
) -> ApiResult<()> {
    let filter = doc! { SESSION_USER_ALIAS: mongodb::bson::to_bson(user_alias).unwrap() };
    session_collection
        .delete_many(filter, None)
        .await
        .map(|_| ())
        .map_err(ApiError::DatabaseError)
}
