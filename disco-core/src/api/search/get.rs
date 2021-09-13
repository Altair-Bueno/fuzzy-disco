use crate::api::result::ApiResult;
use rocket::State;
use mongodb::Collection;
use crate::mongo::user::User;
use crate::mongo::post::Post;
use rocket::serde::json::Json;
use mongodb::bson::doc;
use mongodb::bson::bson;
use chrono::{Utc, DateTime};
use mongodb::bson::DateTime as MongoDateTime;
use crate::api::{USER_CREATION_DATE, POSTS_CREATION_DATE};
use mongodb::bson::to_bson;
use std::option::Option::Some;
use mongodb::bson::from_document;
use rocket::futures::StreamExt;
use mongodb::bson::oid::ObjectId;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use crate::api::data::{ApiUserResponse, ApiPostResponse};
use serde::Deserialize;
use rocket::serde::DeserializeOwned;

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
    s:&str,
    drop:usize,
    get:u8,
    date:&str,
    user_collection: &State<Collection<User>>,
    posts_collection: &State<Collection<Post>>,
) -> ApiResult<Json<Value>> {
    let date : DateTime<Utc> = date.parse()?;
    let date = MongoDateTime::from_chrono(date);
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
    // TODO multi thread async query
    let users: Vec<ApiUserResponse> = search_on_collection(filter_users,user_collection).await?;
    let posts: Vec<ApiPostResponse> = search_on_collection(filter_posts,posts_collection).await?;
    Ok(
        Json(
            json!({"users":users,"posts":posts})
        )
    )
}

async fn search_on_collection <'de,C,T> (
    query:Vec<mongodb::bson::Document>,
    user_collection:&State<Collection<C>>
) -> ApiResult<Vec<T>>
    where
        T: From<C>,
        C: Send + Sync + for<'a> Deserialize<'a>,
{
    let mut out = Vec::new();
    let mut cursor = user_collection.aggregate(query, None).await?;
    while let Some (r) = cursor.next().await{
        let extracted : C = from_document(r?).unwrap();
        let response = T::from(extracted);
        out.push(response)
    }
    Ok(out)
}