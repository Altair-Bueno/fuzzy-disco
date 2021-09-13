use crate::api::users::auth::claims::TokenClaims;
use crate::api::posts::data::EditPostPayload;
use rocket::State;
use crate::mongo::post::Post;
use mongodb::Collection;
use rocket::serde::json::{Json};
use crate::api::result::{ApiResult, ApiError};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use crate::api::{POSTS_AUTHOR, POSTS_ID, POSTS_VISIBILITY};
use mongodb::bson::to_bson;

#[patch("/<id>",format = "json", data="<payload>")]
pub async fn edit_post(
    token: TokenClaims,
    id: &str,
    payload: Json<EditPostPayload>,
    post_collection: &State<Collection<Post>>,
) -> ApiResult<()> {
    let oid: ObjectId = id.parse()?;
    let filter = doc! {POSTS_AUTHOR:to_bson(token.alias()).unwrap(),POSTS_ID:oid};
    let update = doc! {POSTS_VISIBILITY: to_bson(&payload.0).unwrap()};
    let update_result = post_collection.update_one(filter,update, None).await?;

    if update_result.matched_count == 1 {
        Ok(())
    } else {
        Err(ApiError::NotFound("Post"))
    }
}