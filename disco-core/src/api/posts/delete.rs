use mongodb::bson::oid::ObjectId;
use mongodb::bson::to_bson;
use mongodb::{bson::doc, Client, Collection};
use rocket::State;

use crate::api::media::oid_to_path;
use crate::api::result::ApiError::BadRequest;
use crate::api::result::ApiResult;
use crate::api::users::auth::claims::TokenClaims;
use crate::api::{POSTS_AUDIO, POSTS_AUTHOR, POSTS_ID, POSTS_PHOTO};
use crate::mongo::media::Media;
use crate::mongo::post::Post;

/// #  AUTH! `DELETE /api/posts/<id>`
/// Deletes the post. If the user is not the author of the post, a `BadRequest`
/// message will be returned
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
/// | 400 | Bad request |
/// | 404 | Media not found |
/// | 500 | Couldn't connect to database |
///
/// # Example
///
/// `DELETE /api/posts/<id>`
#[delete("/<id>")]
pub async fn delete_post(
    id: &str,
    token: TokenClaims,
    post_collection: &State<Collection<Post>>,
    media_collection: &State<Collection<Media>>,
) -> ApiResult<()> {
    let oid = id.parse::<ObjectId>()?;
    // Delete post
    let filter = doc! {POSTS_ID:oid, POSTS_AUTHOR:to_bson(token.alias()).unwrap()};
    let post = post_collection
        .find_one_and_delete(filter, None)
        .await?
        .ok_or(BadRequest("Couldn't found the associated post"))?;
    // Delete photo
    let filter = doc! {POSTS_PHOTO:post.photo()};
    let photo = media_collection.find_one_and_delete(filter, None).await;
    if let Ok(Some(media)) = photo {
        let _ = rocket::tokio::fs::remove_file(oid_to_path(&media.id().unwrap())).await;
    }
    // Delete audio
    let filter = doc! {POSTS_AUDIO:post.audio()};
    let audio = media_collection.find_one_and_delete(filter, None).await;
    if let Ok(Some(media)) = audio {
        let _ = rocket::tokio::fs::remove_file(oid_to_path(&media.id().unwrap())).await;
    }
    Ok(())
}
