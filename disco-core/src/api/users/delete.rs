use std::option::Option::Some;

use mongodb::Client;
use mongodb::{bson::doc, Collection};
use rocket::futures::StreamExt;
use rocket::http::Status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::State;

use crate::api::media::delete_media;
use crate::api::result::{ApiError, ApiResult};
use crate::api::users::auth::claims::TokenClaims;
use crate::api::{MEDIA_UPLOADED_BY, POSTS_AUTHOR, SESSION_USER_ALIAS, USER_ALIAS};
use crate::mongo::media::Media;
use crate::mongo::post::Post;
use crate::mongo::session::Session;
use crate::mongo::user::User;

/// # AUTH! `DELETE /api/users`
/// Deletes the current authenticated user from the database
///
/// # Returns
/// ## Ok (200)
///
/// ```json
/// {
///     "status": "Ok",
///     "message": "User deleted"
/// }
/// ```
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
/// | 404 | User doesn't exist |
/// | 500 | Couldn't connect to database |
#[delete("/")]
pub async fn delete_user(
    token: TokenClaims,
    user_collection: &State<Collection<User>>,
    media_collection: &State<Collection<Media>>,
    session_collection: &State<Collection<Session>>,
    post_collection: &State<Collection<Post>>,
    mongo_client: &State<Client>,
) -> ApiResult<Value> {
    let bearer_token_alias = token.alias();
    /* TODO removed ACID
        let mut transaction_session = mongo_client.start_session(None).await?;
        let options = TransactionOptions::builder()
            .read_concern(ReadConcern::majority())
            .write_concern(WriteConcern::builder().w(Acknowledgment::Majority).build())
            .build();
        transaction_session.start_transaction(options).await?;
    */
    // Delete the user
    let filter = doc! {USER_ALIAS: mongodb::bson::to_bson(bearer_token_alias).unwrap() };
    let count = user_collection.delete_one(filter, None).await?;
    if count.deleted_count == 0 {
        Err(ApiError::NotFound("User"))
    } else {
        // Delete user sessions
        let filter = doc! { SESSION_USER_ALIAS: mongodb::bson::to_bson(token.alias()).unwrap() };
        session_collection.delete_many(filter, None).await?;
        // Delete user posts
        let filter = doc! { POSTS_AUTHOR:mongodb::bson::to_bson(token.alias()).unwrap() };
        post_collection.delete_many(filter, None).await?;
        // Delete all media uploaded by user
        let filter = doc! { MEDIA_UPLOADED_BY: mongodb::bson::to_bson(token.alias()).unwrap() };
        let mut remove_list = media_collection.find(Some(filter.clone()), None).await?;

        while let Some(next) = remove_list.next().await {
            delete_media(&next?.id().unwrap()).await?
        }
        media_collection.delete_many(filter, None).await?;

        Ok(json!({"status": Status::Ok.reason(), "message": "User deleted"}))
    }
}
