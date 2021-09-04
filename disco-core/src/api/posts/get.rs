use mongodb::bson::doc;
use mongodb::Collection;
use rocket::futures::StreamExt;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, serde_json::json};
use rocket::serde::json::Value;
use rocket::State;

use crate::api::id::Id;
use crate::api::result::ApiResult;
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
#[get("/<id>", format = "json")]
pub async fn get_post_content(
    id: Result<Id,Value>,
    mongo: &State<Collection<Post>>,
) -> ApiResult {
    let oid = match id {
        Ok(x) => x.extract(),
        Err(x) => return status::Custom(Status::BadRequest,x)
    };
    let filter = doc! { "_id": oid };

    let post = match mongo.find_one(Some(filter), None).await {
        Ok(Some(x)) => x,
        Ok(None) => return status::Custom(Status::NotFound, json! ({"message":"Not found"})),
        Err(_) => {
            return status::Custom(
                Status::InternalServerError,
                json! ({"message": "Couldn't load posts from database"}),
            );
        }
    };

    let response = json!({
        "id": post.id().unwrap().to_string(),
        "title": post.title(),
        "caption": post.caption(),
        "author": post.author_id().to_string(),
        "audio": post.audio_path().to_string(),
        "photo": post.photo_path().to_string(),
    });
    status::Custom(Status::Ok,response)
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
pub async fn get_posts(mongo: &State<Collection<Post>>) -> Result<Json<Vec<String>>,status::Custom<Value>> {
    let mut cursor = match mongo.find(None, None).await {
        Ok(cursor) => cursor,
        Err(_) => {
            return Err(status::Custom(
                Status::InternalServerError,
                json! ({"message": "Couldn't connect to database"}),
            ));
        }
    };
    let mut vec = Vec::new();
    while let Some(Ok(post)) = cursor.next().await {
        let id = post.id().unwrap().to_string();
        vec.push(id);
    }
    Ok(Json(vec))
}
