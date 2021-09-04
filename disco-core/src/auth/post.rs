use crate::mongo::user::{User, UserError};
use crate::api::result::ApiResult;
use crate::mongo::post::Post;
use mongodb::Collection;
use rocket::State;
use rocket::response::status::Custom;
use rocket::http::Status;
use rocket::serde::json::serde_json::json;
use crate::auth::Token;
use mongodb::bson::oid::ObjectId;
use rocket::serde::json::Json;
use crate::auth::new_user::NewUser;
use crate::mongo::traits::IntoDocument;


// TODO use other user instead
#[post("/signup",format = "json",data = "<user>")]
pub async fn signup(user: Json<NewUser<'_>>, mongo: &State<Collection<User>>) -> ApiResult {
    let user = match user.0.validate() {
        Ok(x) => x,
        Err(x) => return Custom(Status::BadRequest,json!({"message":x}))
    };
    let mongo_response = mongo.insert_one(user,None).await;
    match mongo_response {
        Ok(x) => {
            Custom(Status::Ok , json!({"message": "User created"}))
        }
        Err(x) =>  Custom(Status::InternalServerError,json!({"message": x.to_string()}))
    }
}