use mongodb::bson::doc;
use mongodb::bson::to_bson;
use mongodb::Collection;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::posts::data::EditPostPayload;
use crate::api::result::{ApiError, ApiResult};
use crate::api::users::auth::claims::TokenClaims;
use crate::api::{POSTS_AUTHOR, POSTS_ID, MEDIA_VISIBILITY, MEDIA_ID};
use crate::mongo::post::Post;
use crate::api::data::ObjectIdWrapper;
use std::option::Option::Some;
use crate::mongo::media::Media;

/// # AUTH! `PATCH /api/posts/<id>`
/// Updates a post with the payload. You must be the author of a post to update
/// it
///
/// # Returns
///
/// # Ok (200)
///
/// ## Err
///
/// ```json
/// {
///     "status": String,
///     "message": String
/// }
/// ```
///
/// | Code | Description |
/// | ---- | ----------- |
/// | 404 | Post not found |
/// | 500 | Couldn't connect to database |
#[patch("/<id>", format = "json", data = "<payload>")]
pub async fn edit_post(
    token: TokenClaims,
    id: ObjectIdWrapper,
    payload: Json<EditPostPayload>,
    post_collection: &State<Collection<Post>>,
    media_collection:&State<Collection<Media>>
) -> ApiResult<()> {
    let oid = id.extract();
    let filter = doc! {POSTS_AUTHOR:token.alias(),POSTS_ID:oid};
    let update = doc! {"$set": to_bson(&payload.0).unwrap()};
    let update_result = post_collection.find_one_and_update(filter, update, None).await?;

    if let Some (post) = update_result{
        if let Some(visibility) = payload.visibility.clone() {
            let filter = doc! {
                "$or": [
                    { MEDIA_ID:post.photo() },
                    { MEDIA_ID:post.audio() }
                ]
            };
            let update = doc! {"$set": {MEDIA_VISIBILITY:visibility}};
            media_collection.update_many(filter,update,None).await?;
        }
        Ok(())
    } else {
        Err(ApiError::NotFound("Post"))
    }
}
