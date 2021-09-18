use std::option::Option::Some;

use mongodb::bson::doc;
use mongodb::bson::from_document;
use mongodb::bson::to_bson;
use mongodb::Collection;
use rocket::futures::StreamExt;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::data::{ApiPostResponse, ApiDate};
use crate::api::result::{ApiError, ApiResult};
use crate::api::users::auth::claims::TokenClaims;
use crate::api::{POSTS_AUTHOR, POSTS_CREATION_DATE, POSTS_VISIBILITY};
use crate::mongo::post::Post;
use crate::mongo::user::Alias;
use crate::mongo::visibility::Visibility;

/// Block size for queries
const BLOCK_SIZE:usize = 40;

/// # `GET /api/users/<id>/posts?block=<usize>&date=<string>`
/// Returns a list of public posts from the given user. The method receives the
/// following query parameters:
///
/// - `block`: Block number to get. Each block has [BLOCK_SIZE] posts
/// - `date`: JSON formatted date, from where to start the query
///
/// # Returns
///
/// ## Ok(200)
///
/// ```json
/// [
///     Post,
///     ...
/// ]
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
/// | ---- | ----------- |
/// | 400 | Bad request |
/// | 404 | User doesn't exist |
/// | 500 | Couldn't connect to database |
#[get("/<alias>/posts?<block>&<date>", rank = 2)]
pub async fn get_posts_from(
    alias: Alias,
    block:usize,
    date: ApiDate,
    posts_collection: &State<Collection<Post>>,
) -> ApiResult<Json<Vec<ApiPostResponse>>> {
    let date = date.extract();
    let query = vec![
        // Look for posts from this author before eq the given date that are
        // public
        doc! { "$match": {
            POSTS_AUTHOR: alias,
            POSTS_CREATION_DATE: { "$lte": date },
            POSTS_VISIBILITY: Visibility::Public
        }},
        // Sort descending
        doc! { "$sort": { POSTS_CREATION_DATE : -1 } },
        doc! { "$skip": to_bson(&(block * BLOCK_SIZE)).unwrap() },
        doc! { "$limit": to_bson(&BLOCK_SIZE).unwrap() },
    ];

    let mut posts_cursor = posts_collection.aggregate(query, None).await?;
    let mut response = Vec::with_capacity(BLOCK_SIZE);

    while let Some(r) = posts_cursor.next().await {
        let post: Post = from_document(r?).unwrap();
        let post_response = ApiPostResponse::from(post);
        response.push(post_response)
    }
    Ok(Json(response))
}

/// # AUTH! `GET /api/users/<id>/posts?private&block=<usize>&date=<string>`
/// Returns a list of private posts from the given user. The user must be the
/// same user that is authenticated. The method receives the following query
/// parameters:
///
/// - `block`: Block number to get. Each block has [BLOCK_SIZE] posts
/// - `date`: JSON formatted date, from where to start the query
///
/// # Returns
///
/// ## Ok(200)
///
/// ```json
/// [
///     Post,
///     ...
/// ]
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
/// | ---- | ----------- |
/// | 400 | Bad request |
/// | 401 | Unauthorised |
/// | 404 | User doesn't exist |
/// | 500 | Couldn't connect to database |
#[get("/<alias>/posts?private&<block>&<date>")]
pub async fn get_private_posts_from(
    token: TokenClaims,
    alias: Alias,
    block: usize,
    date: ApiDate,
    posts_collection: &State<Collection<Post>>,
) -> ApiResult<Json<Vec<ApiPostResponse>>> {
    if alias != *token.alias() {
        return Err(ApiError::Unauthorized("You are not the owner"));
    }
    let date = date.extract();
    let query = vec![
        // Look for posts from this author before eq the given date that are
        // public
        doc! { "$match": {
            POSTS_AUTHOR: alias,
            POSTS_CREATION_DATE: { "$lte": date },
            POSTS_VISIBILITY: Visibility::Private
        }},
        // Sort descending
        doc! { "$sort": { POSTS_CREATION_DATE : -1 } },
        doc! { "$skip": to_bson(&(block * BLOCK_SIZE)).unwrap() },
        doc! { "$limit": to_bson(&BLOCK_SIZE).unwrap() },
    ];

    let mut posts_cursor = posts_collection.aggregate(query, None).await?;
    let mut response = Vec::with_capacity(BLOCK_SIZE);

    while let Some(r) = posts_cursor.next().await {
        let post: Post = from_document(r?).unwrap();
        let post_response = ApiPostResponse::from(post);
        response.push(post_response)
    }
    Ok(Json(response))
}
