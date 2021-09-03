use crate::api::posts_payload::PostPayload;
use mongodb::bson::oid::ObjectId;
use std::str::FromStr;
use rocket::State;
use mongodb::Collection;
use crate::mongo::post::Post;
use mongodb::bson::doc;
use rocket::serde::json::Json;
use rocket::response::status;
use rocket::http::Status;

#[get("/<oid>", format = "json", )]
pub async fn get_post_content(oid:&str, mongo:&State<Collection<Post>>) -> Result<Json<PostPayload>, status::Custom<String>>{
    let not_found = doc! {"message":"Not found"}.to_string();
    let oid = match ObjectId::from_str(oid) {
        Ok(x) => x,
        Err(_) => return Err(status::Custom(Status::NotFound, not_found)),
    };
    let filter = doc! { "_id": oid };
    let post = match mongo.find_one(Some(filter),None).await {
        Ok(Some(x)) => x,
        Ok(_) => return Err(status::Custom(Status::NotFound, not_found)),
        Err(_) => return Err(status::Custom(Status::InternalServerError,doc! {"message": "Server error"}.to_string()))
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