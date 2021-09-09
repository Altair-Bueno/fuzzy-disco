use mongodb::bson::doc;
use mongodb::Collection;
use rocket::State;

use crate::api::result::ApiError;
use crate::mongo::user::{Alias, User};

/// /api/users/auth
pub mod auth;
mod data;
/// DELETE /api/users
pub mod delete;
/// GET /api/users/
pub mod get;
/// PUT /api/users
pub mod post;

// helper functions

async fn locate_user(alias: &Alias, mongo: &State<Collection<User>>) -> Result<User, ApiError> {
    let result = mongo
        .find_one(doc! {"alias": mongodb::bson::to_bson(alias).unwrap() }, None)
        .await?;
    match result {
        None => Err(ApiError::NotFound("User")),
        Some(x) => Ok(x),
    }
}
