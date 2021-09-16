use std::option::Option::Some;

use mongodb::bson::doc;
use mongodb::bson::from_document;
use mongodb::bson::to_bson;
use mongodb::Collection;
use rocket::futures::StreamExt;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket::State;
use serde::Deserialize;

use crate::api::data::{ApiPostResponse, ApiUserResponse, ApiDate};
use crate::api::result::ApiResult;
use crate::api::{POSTS_CREATION_DATE, USER_CREATION_DATE};
use crate::mongo::post::Post;
use crate::mongo::user::User;

/// # `GET /api/search?s=<string>&drop=<usize>&get=<u8>&date=<string>`
/// Returns matches for the given search string. The method receives the
/// following query parameters
///
/// - `s`: Search string
/// - `drop`: Number of posts/users we want to skip. This avoids repetition on
/// future queries
/// - `get`: Number of posts we want to retrieve. The max is 255 posts
/// - `date`: JSON formatted date, from where to start the query
///
/// # Returns
/// ## Ok(200)
///
/// ```json
/// {
///     "users": [Users],
///     "posts": [Posts],
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
/// | ---- | ----------- |
/// | 400 | Bad request |
/// | 404 | User doesn't exist |
/// | 500 | Couldn't connect to database |
#[get("/?<s>&<drop>&<get>&<date>")]
pub async fn search(
    s: &str,
    drop: usize,
    get: u8,
    date: ApiDate,
    user_collection: &State<Collection<User>>,
    posts_collection: &State<Collection<Post>>,
) -> ApiResult<Json<Value>> {
    let date = date.extract();
    let filter_users = vec![
        doc! { "$match": {
            "$text": {"$search": s},
            USER_CREATION_DATE:{ "$lte": date }
        }},
        doc! { "$sort": { "score": { "$meta": "textScore" } } },
        doc! { "$skip": to_bson(&drop).unwrap() },
        doc! { "$limit": to_bson(&get).unwrap() },
    ];
    let filter_posts = vec![
        doc! { "$match": {
            "$text": {"$search": s},
            POSTS_CREATION_DATE:{ "$lte": date }
        }},
        doc! { "$sort": { "score": { "$meta": "textScore" } } },
        doc! { "$skip": to_bson(&drop).unwrap() },
        doc! { "$limit": to_bson(&get).unwrap() },
    ];
    // TODO async search on parallel using `sope` threads
    // Note: AsyncDrop does not exist, neither scoped async tasks. It is not
    // currently possible to perform two separate queries on different tasks
    // without wrapping the client on `Ark`
    let users: Vec<ApiUserResponse> = search_on_collection(filter_users, user_collection).await?;
    let posts: Vec<ApiPostResponse> = search_on_collection(filter_posts, posts_collection).await?;
    Ok(Json(json!({"users":users,"posts":posts})))
}

async fn search_on_collection<'de, C, T>(
    query: Vec<mongodb::bson::Document>,
    user_collection: &State<Collection<C>>,
) -> ApiResult<Vec<T>>
where
    T: From<C>,
    C: Send + Sync + for<'a> Deserialize<'a>,
{
    let mut out = Vec::new();
    let mut cursor = user_collection.aggregate(query, None).await?;
    while let Some(r) = cursor.next().await {
        let extracted: C = from_document(r?).unwrap();
        let response = T::from(extracted);
        out.push(response)
    }
    Ok(out)
}
