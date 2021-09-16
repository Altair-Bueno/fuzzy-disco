use mongodb::bson::doc;
use mongodb::Collection;
use rocket::State;

use crate::api::result::{ApiError, ApiResult};
use crate::api::USER_ALIAS;
use crate::mongo::user::{Alias, User};

/// /api/users/auth
pub mod auth;
/// Data Structures used on this module
mod data;
/// DELETE /api/users
pub mod delete;
/// GET /api/users/
pub mod get;
/// PUT /api/users
pub mod post;
/// /api/users/\<alias>/posts
pub mod posts;

// helper functions

async fn locate_user(alias: &Alias, mongo: &State<Collection<User>>) -> ApiResult<User> {
    let result = mongo
        .find_one(
            doc! {USER_ALIAS: alias },
            None,
        )
        .await?;
    match result {
        None => Err(ApiError::NotFound("User")),
        Some(x) => Ok(x),
    }
}
