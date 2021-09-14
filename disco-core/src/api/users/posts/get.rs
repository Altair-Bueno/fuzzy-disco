use std::option::Option::Some;

use chrono::{DateTime, Utc};
use mongodb::bson::doc;
use mongodb::bson::from_document;
use mongodb::bson::to_bson;
use mongodb::bson::DateTime as MongoDateTime;
use mongodb::Collection;
use rocket::futures::StreamExt;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::data::ApiPostResponse;
use crate::api::result::{ApiError, ApiResult};
use crate::api::users::auth::claims::TokenClaims;
use crate::api::{POSTS_AUTHOR, POSTS_CREATION_DATE, POSTS_VISIBILITY};
use crate::mongo::post::Post;
use crate::mongo::user::Alias;
use crate::mongo::visibility::Visibility;

/// # `GET /api/users/<id>/posts?drop=<usize>&get=<u8>&date=<string>`
/// Returns a list of public posts from the given user. The method receives the
/// following query parameters:
///
/// - `drop`: Number of posts we want to skip. This avoids repetition on future
/// queries
/// - `get`: Number of posts we want to retrieve. The max is 255 posts
/// - `date`: JSON formatted date, from where to start the query
///
/// # Returns
///
/// ## Ok(200)
///
/// ```json
/// [
///     Post,
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
#[get("/<alias>/posts?<drop>&<get>&<date>", rank = 2)]
pub async fn get_posts_from(
    alias: &str,
    drop: usize,
    get: u8,
    date: &str,
    posts_collection: &State<Collection<Post>>,
) -> ApiResult<Json<Vec<ApiPostResponse>>> {
    // TODO test me
    let alias: Alias = alias.parse()?;
    let date: DateTime<Utc> = date.parse()?;
    let date = MongoDateTime::from_chrono(date);
    let query = vec![
        // Look for posts from this author before eq the given date that are
        // public
        doc! { "$match": {
            POSTS_AUTHOR: to_bson(&alias).unwrap(),
            POSTS_CREATION_DATE: { "$lte": date },
            POSTS_VISIBILITY: to_bson(&Visibility::Public).unwrap()
        }},
        // Sort descending
        doc! { "$sort": { POSTS_CREATION_DATE : -1 } },
        doc! { "$skip": to_bson(&drop).unwrap() },
        doc! { "$limit": to_bson(&get).unwrap() },
    ];

    let mut posts_cursor = posts_collection.aggregate(query, None).await?;
    let mut response = Vec::with_capacity(get as usize);

    while let Some(r) = posts_cursor.next().await {
        let post: Post = from_document(r?).unwrap();
        let post_response = ApiPostResponse::from(post);
        response.push(post_response)
    }
    Ok(Json(response))
}

/// # AUTH! `GET /api/users/<id>/posts?private&drop=<usize>&get=<u8>&date=<string>`
/// Returns a list of private posts from the given user. The user must be the
/// same user that is authenticated. The method receives the following query
/// parameters:
///
/// - `drop`: Number of posts we want to skip. This avoids repetition on future
/// queries
/// - `get`: Number of posts we want to retrieve. The max is 255 posts
/// - `date`: JSON formatted date, from where to start the query
///
/// # Returns
///
/// ## Ok(200)
///
/// ```json
/// [
///     Post,
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
#[get("/<alias>/posts?private&<drop>&<get>&<date>")]
pub async fn get_private_posts_from(
    token: TokenClaims,
    alias: &str,
    drop: usize,
    get: u8,
    date: &str,
    posts_collection: &State<Collection<Post>>,
) -> ApiResult<Json<Vec<ApiPostResponse>>> {
    // TODO test me
    let alias: Alias = alias.parse()?;
    if alias != *token.alias() {
        return Err(ApiError::Unauthorized("You are not the owner"));
    }

    let date: DateTime<Utc> = date.parse()?;
    let date = MongoDateTime::from_chrono(date);
    let query = vec![
        // Look for posts from this author before eq the given date that are
        // public
        doc! { "$match": {
            POSTS_AUTHOR: to_bson(&alias).unwrap(),
            POSTS_CREATION_DATE: { "$lte": date },
            POSTS_VISIBILITY: to_bson(&Visibility::Private).unwrap()
        }},
        // Sort descending
        doc! { "$sort": { POSTS_CREATION_DATE : -1 } },
        doc! { "$skip": to_bson(&drop).unwrap() },
        doc! { "$limit": to_bson(&get).unwrap() },
    ];

    let mut posts_cursor = posts_collection.aggregate(query, None).await?;
    let mut response = Vec::with_capacity(get as usize);

    while let Some(r) = posts_cursor.next().await {
        let post: Post = from_document(r?).unwrap();
        let post_response = ApiPostResponse::from(post);
        response.push(post_response)
    }
    Ok(Json(response))
}
