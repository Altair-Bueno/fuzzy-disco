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
use crate::api::search::data::Window;

/// # `GET /api/search?s=<string>&date=<string>&user.drop=<usize>&user.get=<u8>&post.drop=<usize>&post.get=<u8>`
/// Returns matches for the given search string. The method receives the
/// following query parameters
///
/// - `s`: Search string
/// - `date`: JSON formatted date, from where to start the query
/// - `drop`: Number of posts/users we want to skip. This avoids repetition on
/// future queries. Defaults to 0 if no value is provided
/// - `get`: Number of posts/users we want to retrieve. The max is 255 elements.
/// Defaults to 0 if no value is provided
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
#[get("/?<s>&<user>&<post>&<date>")]
pub async fn search(
    s: &str,
    user:Window,
    post:Window,
    date: ApiDate,
    user_collection: &State<Collection<User>>,
    posts_collection: &State<Collection<Post>>,
) -> ApiResult<Json<Value>> {
    let date = date.extract();

    // TODO async search on parallel using `scope` threads
    // Note: AsyncDrop does not exist, neither scoped async tasks. It is not
    // currently possible to perform two separate queries on different tasks
    // without wrapping the client on `Ark`
    let users: Option<Vec<ApiUserResponse>> = if user.get >0 {
        let filter_users = vec![
            doc! { "$match": {
            "$text": {"$search": s},
            USER_CREATION_DATE:{ "$lte": date }
        }},
            doc! { "$sort": { "score": { "$meta": "textScore" } } },
            doc! { "$skip": to_bson(&user.drop).unwrap() },
            doc! { "$limit": to_bson(&user.get).unwrap() },
        ];
        Some(search_on_collection(filter_users, user_collection).await?)
    } else {
        None
    };
    let posts: Option<Vec<ApiPostResponse>> = if post.get > 0 {
        let filter_posts = vec![
            doc! { "$match": {
            "$text": {"$search": s},
            POSTS_CREATION_DATE:{ "$lte": date }
        }},
            doc! { "$sort": { "score": { "$meta": "textScore" } } },
            doc! { "$skip": to_bson(&post.drop).unwrap() },
            doc! { "$limit": to_bson(&post.get).unwrap() },
        ];
        Some(search_on_collection(filter_posts, posts_collection).await?)
    } else {
        None
    };
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
