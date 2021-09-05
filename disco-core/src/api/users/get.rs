use crate::api::result::ApiResult;
use rocket::State;
use mongodb::Collection;
use crate::mongo::user::{User, Alias, UserError};
use mongodb::bson::doc;
use rocket::response::status::Custom;
use rocket::http::Status;
use rocket::serde::json::serde_json::json;

#[get("/<alias>")]
pub async fn get_user_info(alias:&str, mongo: &State<Collection<User>>) -> ApiResult{
    let alias = match alias.parse::<Alias>() {
        Ok(x) => x,
        Err(_) => return Custom(Status::BadRequest, json!({"status":Status::BadRequest.reason(),"message": "Invalid alias"}))
    };
    let result = mongo.find_one(doc! {"alias": alias.to_string() }, None).await;
    match result {
        Ok(Some(x)) => {
            Custom(Status::Ok, json!({
                "alias": x.alias(),
                "posts": x.posts(),
            }))
        }
        Ok(None) => Custom(Status::NotFound,json!({"status":Status::NotFound.reason(),"message": "User doesn't exist"})),
        Err(_) => Custom(Status::InternalServerError, json!({"status": Status::InternalServerError.reason(),"message": "Couldn't connect to database"}))
    }
}