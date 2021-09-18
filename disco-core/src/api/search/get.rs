use std::option::Option::Some;

use mongodb::bson::doc;
use mongodb::bson::from_document;
use mongodb::bson::to_bson;
use mongodb::Collection;
use rocket::futures::StreamExt;
use rocket::State;
use serde::Deserialize;

use crate::api::data::{ApiPostResponse, ApiUserResponse, ApiDate};
use crate::api::result::ApiResult;
use crate::api::{POSTS_CREATION_DATE, USER_CREATION_DATE, USER_ALIAS, POSTS_VISIBILITY, POSTS_TITLE, POSTS_CAPTION};
use crate::mongo::post::Post;
use crate::mongo::user::User;
use crate::mongo::visibility::Visibility;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, RedisResult};


/// 2^16ms = 65536ms = 1.0922667m. It allows easy extraction by `and` operation
const SEARCH_CACHE_TTL:i64 = 65536;
const BLOCK_SIZE:usize = 20;

/// # `GET /api/search/<r>?s=<string>&date=<string>&block=<usize>`
/// Search on database
///
/// - `r`: The request type. It can be `user` or `post`
/// - `s`: Regex to search
/// - `date`: Date from where to start the search. Use this to avoid race
/// conditions
/// - `block`: The block to look for. A block is a sequence of [BLOCK_SIZE]
/// elements. For example, if you want the 30 latest posts, you'll need 3 calls
/// to this api with `block = [0,1,2]`
///
/// # Returns
/// ## Ok(200)
///
/// ```json
/// [
///     User/Post,
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
///
/// # Example
///
/// `GET /api/search/post?s=hello&block=2&date=2021-09-18T11%3A30%3A51.511Z`
///
/// ## Response
///
/// ```json
/// []
/// ```
#[get("/post?<s>&<date>&<block>",format = "json")]
pub async fn search_post(
    s: &str,
    date: ApiDate,
    block:usize,
    post_collection: &State<Collection<Post>>,
    redis_cache: &State<MultiplexedConnection>
) -> ApiResult<String> {
    let date = date.extract().timestamp_millis();
    let date = mongodb::bson::DateTime::from_millis(date - (date % SEARCH_CACHE_TTL));
    let key = format!("post:{}/{}/{}",s,date,block);
    let mut redis_cache = redis_cache.inner().clone();
    if let Ok(Some(hit)) = redis_cache.get(&key).await {
        Ok(hit)
    } else {
        let filter_posts = vec![
            doc! { "$match": {
                POSTS_CREATION_DATE:{ "$lte": date },
                POSTS_VISIBILITY : Visibility::Public,
                "$or": [
                    {POSTS_TITLE: mongodb::bson::Regex{ pattern: s.to_string(), options: "".to_string() }},
                    {POSTS_CAPTION: mongodb::bson::Regex{ pattern: s.to_string(), options: "".to_string() }}
                ]
            }},
            doc! { "$sort": {POSTS_CREATION_DATE: -1 } },
            doc! { "$skip": to_bson(&(block * BLOCK_SIZE)).unwrap() },
            doc! { "$limit": to_bson(&BLOCK_SIZE).unwrap() },
        ];
        let search : Vec<ApiPostResponse> = search_on_collection(filter_posts,post_collection).await?;
        let serialized = serde_json::to_string(&search).unwrap();
        let _ : RedisResult<()> = redis_cache.set(&key,&serialized).await;
        Ok(serialized)
    }
}
#[get("/user?<s>&<date>&<block>", format = "json")]
pub async fn search_user(
    s: &str,
    date: ApiDate,
    block:usize,
    user_collection: &State<Collection<User>>,
    redis_cache: &State<MultiplexedConnection>
) -> ApiResult<String> {
    let date = date.extract().timestamp_millis();
    let date = mongodb::bson::DateTime::from_millis(date - (date % SEARCH_CACHE_TTL));
    let mut redis_cache = redis_cache.inner().clone();
    let key = format!("user:{}/{}/{}",s,date,block);
    if let Ok(Some(hit)) = redis_cache.get(&key).await {
        Ok(hit)
    } else {
        let filter_users = vec![
            doc! { "$match": {
                USER_CREATION_DATE:{ "$lte": date },
                USER_ALIAS: mongodb::bson::Regex{ pattern: s.to_string(), options: "".to_string() }
            }},
            doc! { "$sort": { USER_CREATION_DATE: -1 } },
            doc! { "$skip": to_bson(&(block * BLOCK_SIZE)).unwrap() },
            doc! { "$limit": to_bson(&BLOCK_SIZE).unwrap() },
        ];
        let search: Vec<ApiUserResponse> = search_on_collection(filter_users, user_collection).await?;
        let serialized = serde_json::to_string(&search).unwrap();
        let _ :RedisResult<()> = redis_cache.set(&key,&serialized).await;
        Ok(serialized)
    }
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
