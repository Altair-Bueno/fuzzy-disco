use mongodb::bson::doc;
use mongodb::bson::to_bson;
use mongodb::Collection;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::posts::data::EditPostPayload;
use crate::api::result::{ApiError, ApiResult};
use crate::api::users::auth::claims::TokenClaims;
use crate::api::{POSTS_AUTHOR, POSTS_ID};
use crate::mongo::post::Post;
use crate::api::data::ObjectIdWrapper;

#[patch("/<id>", format = "json", data = "<payload>")]
pub async fn edit_post(
    token: TokenClaims,
    id: ObjectIdWrapper,
    payload: Json<EditPostPayload>,
    post_collection: &State<Collection<Post>>,
) -> ApiResult<()> {
    let oid = id.extract();
    let filter = doc! {POSTS_AUTHOR:token.alias(),POSTS_ID:oid};
    let update = doc! {"$set": to_bson(&payload.0).unwrap()};
    let update_result = post_collection.update_one(filter, update, None).await?;

    if update_result.matched_count == 1 {
        // TODO: media should become private
        Ok(())
    } else {
        Err(ApiError::Unauthorized("You are not allowed to modify this post"))
    }
}
