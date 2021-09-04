use std::collections::HashMap;
use std::str::FromStr;

use maplit::hashmap;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use rocket::futures::StreamExt;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;

use crate::mongo::post::Post;

/// # `GET /api/posts/<id>`
/// Returns information for a given post. It expects a well formated string
/// that identifies a post
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
/// }
/// ```
///
/// ## Err
/// ```json
/// {
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
///  "author": "6131f8946c2cc66344ef2a86",
///  "id": "6132137e6c2cc66344ef2a88",
///  "caption": "Hisoka wants gon booty",
///  "title": "Hunter x Hunter"
///}
/// ```
#[get("/<oid>", format = "json")]
pub async fn get_post_content(
    oid: &str,
    mongo: &State<Collection<Post>>,
) -> Result<Json<HashMap<&'static str, String>>, status::Custom<String>> {
    let oid = match ObjectId::from_str(oid) {
        Ok(x) => x,
        Err(_) => {
            return Err(status::Custom(
                Status::BadRequest,
                doc! {"message": "Invalid ID"}.to_string(),
            ))
        }
    };

    let filter = doc! { "_id": oid };
    let post = match mongo.find_one(Some(filter), None).await {
        Ok(Some(x)) => x,
        Ok(None) => {
            return Err(status::Custom(
                Status::NotFound,
                doc! {"message":"Not found"}.to_string(),
            ))
        }
        Err(_) => {
            return Err(status::Custom(
                Status::InternalServerError,
                doc! {"message": "Couldn't load posts from database"}.to_string(),
            ))
        }
    };

    let resonse = hashmap! {
        "id" => post.id().unwrap().to_string(),
        "title" => post.title().to_string(),
        "caption" => post.caption().to_string(),
        "author" => post.author_id().to_string(),
        "audio" => post.audio_path().to_string(),
        "photo" => post.photo_path().to_string(),
    };

    Ok(Json(resonse))
}

/// #!DEBUG
/// # `GET /api/posts`
///
/// Returns all postID on the database as a single vector of strings
///
/// ```json
/// [
///     String,
///     ...
/// ]
/// ```
///
/// # Example
///
/// `GET /api/posts`
///
/// ```json
/// [
///  "6131f8946c2cc66344ef2a86",
///  "6132137e6c2cc66344ef2a88",
///  "613213e26c2cc66344ef2a89",
///  "613214076c2cc66344ef2a8a"
///]
/// ```
#[get("/", format = "json")]
pub async fn get_posts(
    mongo: &State<Collection<Post>>,
) -> Result<Json<Vec<String>>, status::Custom<String>> {
    let mut cursor = match mongo.find(None, None).await {
        Ok(cursor) => cursor,
        Err(_) => {
            return Err(status::Custom(
                Status::InternalServerError,
                doc! {"message": "Couldn't connect to database"}.to_string(),
            ))
        }
    };
    let mut vec = Vec::new();
    while let Some(Ok(post)) = cursor.next().await {
        let id = post.id().unwrap().to_string();
        vec.push(id);
    }
    Ok(Json(vec))
}
