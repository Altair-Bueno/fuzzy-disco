use std::str::FromStr;

use mongodb::bson::doc;
use rocket::tokio::fs::File;
use rocket::State;

use crate::api::media::oid_to_path;
use crate::api::result::{ApiError, ApiResult};
use crate::api::users::auth::token::claims::TokenClaims;
use crate::mongo::media::{Media, Status};
use crate::mongo::visibility::Visibility;
use crate::api::{MEDIA_ID, MEDIA_STATUS};

/// # `GET /api/media/<id>`
/// Returns the requested media by its id
///
/// > Note: Requesting unclaimed media will return `404 Not found`
///
/// # Auth behaviour
/// - If the user is not authenticated, only public media is available
/// - If the user is authenticated, private media uploaded by them are available
/// too
///
/// # Returns
/// ## Ok (200)
///
/// ## Err
/// ```json
/// {
///     "status": String,
///     "message": String
/// }
/// ```
///
/// | Code | Description |
/// | -----| ----------- |
/// | 401 | Unauthorised. Private media |
/// | 404 | Media not found or unclaimed |
/// | 500 | Couldn't connect to database |
///
#[get("/<id>")]
pub async fn get_media_auth(
    id: &str,
    token: TokenClaims,
    mongo_media: &State<mongodb::Collection<Media>>,
) -> ApiResult<File> {
    let oid = mongodb::bson::oid::ObjectId::from_str(id)?;
    let filter = doc! {MEDIA_ID: oid, MEDIA_STATUS : mongodb::bson::to_bson(&Status::Assigned).unwrap() };
    let media = mongo_media
        .find_one(filter, None)
        .await?
        .ok_or(ApiError::NotFound("Media"))?;
    let condition = (*media.visibility() == Visibility::Public) ||
        (token.alias() == media.uploaded_by());

    if condition {
        Ok(rocket::tokio::fs::File::open(oid_to_path(&oid)).await?)
    } else {
        Err(ApiError::Unauthorized("Private media"))
    }
}
#[get("/<id>", rank = 2)]
pub async fn get_media(
    id: &str,
    mongo_media: &State<mongodb::Collection<Media>>,
) -> ApiResult<File> {
    let oid = mongodb::bson::oid::ObjectId::from_str(id)?;
    let filter = doc! {MEDIA_ID: oid, MEDIA_STATUS : mongodb::bson::to_bson(&Status::Assigned).unwrap() };
    let media = mongo_media
        .find_one(filter, None)
        .await?
        .ok_or(ApiError::NotFound("Media"))?;

    if *media.visibility() == Visibility::Public {
        Ok(rocket::tokio::fs::File::open(oid_to_path(&oid)).await?)
    } else {
        Err(ApiError::Unauthorized("Private media"))
    }
}