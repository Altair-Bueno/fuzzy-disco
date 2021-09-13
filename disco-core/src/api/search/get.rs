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

// TODO check if it works and add documentation
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
        // Remove all fields except for the ObjectID
        doc! { "$project": { "_id": 1 } }
    ];
    let filter_posts = vec![
        doc! { "$match": {
            "$text": {"$search": s},
            POSTS_CREATION_DATE:{ "$lte": date }
        }},
        doc! { "$sort": { "score": { "$meta": "textScore" } } },
        doc! { "$skip": to_bson(&drop).unwrap() },
        doc! { "$limit": to_bson(&get).unwrap() },
        // Remove all fields except for the ObjectID
        doc! { "$project": { "_id": 1 } }
    ];
    let mut response_posts = Vec::with_capacity(get as usize);
    let mut posts_cursor = posts_collection.aggregate(filter_posts,None).await?;
    while let Some (r) = posts_cursor.next().await{
        let id = from_document::<Payload>(r?).unwrap().id;
        response_posts.push(id.to_string());
    }
    let mut response_users = Vec::with_capacity(get as usize);
    let mut user_cursor = user_collection.aggregate(filter_users,None).await?;
    while let Some (r) = user_cursor.next().await{
        let id = from_document::<Payload>(r?).unwrap().id;
        response_users.push(id.to_string());
    }
    Ok(Json(json! ({
            "posts": response_posts,
            "users": response_users
        })
    ))
}

use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct Payload {
    #[serde(rename="_id")]
    id: ObjectId
}