use rocket::response::content::Json;
use crate::api::post::Post as ResponsePost;
use crate::mongo::post::{Post as DocPost, Post};
use rocket::State;
use mongodb::Collection;
use std::str::FromStr;
use mongodb::bson::doc;

/*
#[get("/<id>")]
pub fn post(id:String) -> Json<crate::api::post::Post> {
    todo!()
}*/

/*
#[get("/<id>", format = "json")]
pub async fn get_post(id:&str, posts: &State<Collection<DocPost>>) -> Result<ResponsePost,String> {
    let oid = mongodb::bson::oid::ObjectId::from_str(id);
    let filter = doc! { "_id": oid };
    let found = posts.find_one(Some(filter), None).await?;
    match found {
        None => Err(mongodb::error::Error::from("Err")),
        Some(x) => Ok(ResponsePost::from(x))
    }
    todo!()
}
*/