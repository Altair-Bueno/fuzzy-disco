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
/// | 404 | Media not found |
/// | 500 | Couldn't connect to database |
///
#[get("/<id>")]
pub async fn get_media(
    id: &str,
    token: Option<TokenClaims>,
    mongo_media: &State<mongodb::Collection<Media>>,
) -> ApiResult<File> {
    let oid = mongodb::bson::oid::ObjectId::from_str(id)?;
    let filter = doc! {MEDIA_ID: oid, MEDIA_STATUS : mongodb::bson::to_bson(&Status::Assigned).unwrap() };
    let media = mongo_media
        .find_one(filter, None)
        .await?
        .ok_or(ApiError::NotFound("Media"))?;

    let cannot_see = (*media.visibility() == Visibility::Private)
        && token
            .map(|x| x.alias() != media.uploaded_by())
            .unwrap_or(true);

    if cannot_see {
        Err(ApiError::Unauthorized("Private media"))
    } else {
        Ok(rocket::tokio::fs::File::open(oid_to_path(&oid)).await?)
    }
}
