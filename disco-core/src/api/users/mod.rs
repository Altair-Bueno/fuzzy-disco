use crate::mongo::user::{Alias, User};
use rocket::State;
use mongodb::Collection;
use crate::api::result::ApiError;
use mongodb::bson::doc;

/// /api/users/auth
pub mod auth;
mod data;
/// DELETE /api/users
pub mod delete;
/// GET /api/users/
pub mod get;
/// PUT /api/users
pub mod put;

// helper functions

async fn locate_user(alias: &Alias, mongo: &State<Collection<User>>) -> Result<User, ApiError> {
    let result = mongo
        .find_one(doc! {"alias": alias.to_string() }, None)
        .await?;
    match result {
        None => Err(ApiError::NotFound("User")),
        Some(x) => Ok(x),
    }
}
