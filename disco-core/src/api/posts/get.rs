use crate::api::posts_payload::PostPayload;
use crate::mongo::post::Post;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use rocket::futures::StreamExt;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use std::str::FromStr;

#[get("/<oid>", format = "json")]
pub async fn get_post_content(
    oid: &str,
    mongo: &State<Collection<Post>>,
) -> Result<Json<PostPayload>, status::Custom<String>> {
    let not_found = doc! {"message":"Not found"}.to_string();
    let oid = match ObjectId::from_str(oid) {
        Ok(x) => x,
        Err(_) => return Err(status::Custom(Status::NotFound, not_found)),
    };
    let filter = doc! { "_id": oid };
    let post = match mongo.find_one(Some(filter), None).await {
        Ok(Some(x)) => x,
        Ok(None) => return Err(status::Custom(Status::NotFound, not_found)),
        Err(_) => {
            return Err(status::Custom(
                Status::InternalServerError,
                doc! {"message": "Couldn't load post from database"}.to_string(),
            ))
        }
    };

    let mut post_response = PostPayload::new();
    post_response.set_audio_path(Some(post.audio_path().to_string()));
    post_response.set_author_id(Some(post.author_id().to_string()));
    post_response.set_caption(Some(post.caption().to_string()));
    post_response.set_id(Some(post.id().unwrap().to_string()));
    post_response.set_photo_path(Some(post.photo_path().to_string()));
    post_response.set_title(Some(post.title().to_string()));

    Ok(Json(post_response))
}

#[get("/", format = "json")]
pub async fn get_posts(
    mongo: &State<Collection<Post>>,
) -> Result<Json<Vec<PostPayload>>, status::Custom<String>> {
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
    while let Some(post) = cursor.next().await {
        let mut payload = PostPayload::new();
        match post {
            Ok(post) => {
                payload.set_id(post.id().map(|x| x.to_string()));
                vec.push(payload);
            }
            Err(_) => break,
        }
    }
    Ok(Json(vec))
}
