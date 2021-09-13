use mongodb::bson::doc;
use mongodb::Collection;
use rocket::serde::json::Value;
use rocket::serde::json::{serde_json::json, Json};
use rocket::State;

use crate::api::result::{ApiResult, ApiError};
use crate::mongo::post::Post;
use std::str::FromStr;
use crate::api::POSTS_ID;
use mongodb::bson::to_bson;
use crate::api::users::auth::claims::TokenClaims;
use crate::mongo::visibility::Visibility;

/// # `GET /api/posts/<id>`
/// Returns information for a given post. It expects a well formated string
/// that identifies a post.
///
///
/// # Auth behaviour
/// - If the user is not authenticated, only public post are available
/// - If the user is authenticated, private posts uploaded by them are available
/// too
///
/// # Returns
/// ## Ok (200)
///
/// ```json
/// {
///     "id": String,
///     "title": String,
///     "caption": String,
///     "author": String.
///     "audio": String,
///     "photo": String,
///     "visibility": Visibility,
///     "creation_date": String
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
/// | 400 | `id` isn't correctly formated |
/// | 404 | Post doesn't exist |
/// | 500 | Couldn't connect to database |
///
/// # Example
///
/// `GET /api/posts/6132137e6c2cc66344ef2a88`
///
/// ```json
/// {
///  "audio": "6032137e6c2cc66244ef2a88",
///  "photo": "5032137e6c2cc66244ef2a88",
///  "author": "Altair-Bueno",
///  "id": "6132137e6c2cc66344ef2a88",
///  "caption": "Hisoka wants gon booty",
///  "title": "Hunter x Hunter",
///  "visibility": "Public",
///  "creation_date": "2021-09-06 16:13:02.797 UTC"
///}
/// ```
#[get("/<id>", format = "json" , rank = 2)]
pub async fn get_post_content(id: &str, mongo: &State<Collection<Post>>) -> ApiResult<Json<Value>> {
    let post = get_post(id,mongo).await?;
    if *post.visibility() == Visibility::Public {
        Ok(Json(generate_response(&post)))
    } else {
        Err(ApiError::Unauthorized("Private post"))
    }
}

#[get("/<id>", format = "json")]
pub async fn get_post_content_auth(
    token:TokenClaims,
    id: &str,
    mongo: &State<Collection<Post>>
) -> ApiResult<Json<Value>> {
    let post = get_post(id,mongo).await?;
    let condition = (*post.visibility() == Visibility::Public) ||
        (token.alias() == post.author());

    if condition {
        Ok(Json(generate_response(&post)))
    } else {
        Err(ApiError::Unauthorized("Private post"))
    }
}

async fn get_post(id: &str, mongo: &State<Collection<Post>>) -> ApiResult<Post> {
    let oid = mongodb::bson::oid::ObjectId::from_str(id)?;
    let filter = doc! {POSTS_ID:oid};
    mongo.find_one(Some(filter),None)
        .await?
        .ok_or(ApiError::NotFound("Post"))
}
fn generate_response(post:&Post) -> Value {
    json!({
        // `unwrap` here is safe, represents _id from db
        "id": post.id().unwrap().to_string(),
        "title": to_bson(post.title()).unwrap(),
        "caption": to_bson(post.caption()).unwrap(),
        "author": to_bson(post.author()).unwrap(),
        "audio": post.audio().to_string(),
        "photo": post.photo().to_string(),
        "visibility": to_bson(post.visibility()).unwrap(),
        "creation_date": post.creation_date().to_string()
    })
}
