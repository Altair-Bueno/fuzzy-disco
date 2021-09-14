use crate::api::result::ApiResult;
use mongodb::bson::oid::ObjectId;
use crate::api::users::auth::claims::TokenClaims;
use rocket::State;
use crate::mongo::post::Post;
use crate::mongo::media::Media;
use crate::api::{POSTS_ID, POSTS_PHOTO, POSTS_AUDIO, POSTS_AUTHOR};
use crate::api::result::ApiError::{BadRequest};
use crate::api::media::oid_to_path;
use mongodb::bson::to_bson;

use mongodb::{
    Client,
    bson::doc,
    options::{Acknowledgment, ReadConcern, TransactionOptions, WriteConcern},
    Collection,
};

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
    id:&str,
    token: TokenClaims,
    post_collection: &State<Collection<Post>>,
    media_collection: &State<Collection<Media>>,
    mongo_client: &State<Client>
) -> ApiResult<()> {
    let oid = id.parse::<ObjectId>()?;

    /* TODO acid
    let mut transaction_session = mongo_client.start_session(None).await?;
    let options = TransactionOptions::builder()
        .read_concern(ReadConcern::majority())
        .write_concern(WriteConcern::builder().w(Acknowledgment::Majority).build())
        .build();
    transaction_session.start_transaction(options).await?;*/
    // Delete post
    let filter = doc! {POSTS_ID:oid, POSTS_AUTHOR:to_bson(token.alias()).unwrap()};
    let post = post_collection.find_one_and_delete(filter,None)
        .await?
        .ok_or(BadRequest("Couldn't found the associated post"))?;
    // Delete photo
    let filter = doc! {POSTS_PHOTO:post.photo()};
    let photo = media_collection.find_one_and_delete(filter,None).await?;
    if let Some(media) = photo {
        rocket::tokio::fs::remove_file(oid_to_path(&media.id().unwrap())).await?;
    }
    // Delete audio
    let filter = doc! {POSTS_AUDIO:post.audio()};
    let audio = media_collection.find_one_and_delete(filter,None).await?;
    if let Some(media) = audio {
        rocket::tokio::fs::remove_file(oid_to_path(&media.id().unwrap())).await?;
    }
    Ok(())
}