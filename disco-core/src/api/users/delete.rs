use crate::api::result::ApiResult;
use crate::mongo::user::{Alias, User};
use mongodb::bson::doc;
use mongodb::Collection;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::State;
use crate::api::users::auth::token::claims::TokenClaims;

#[delete("/<alias>")]
pub async fn delete_user(
    alias: Alias,
    token: TokenClaims,
    mongo: &State<Collection<User>>,
) -> ApiResult {
    let bearer_token_alias = token.alias();
    if *bearer_token_alias == alias {
        let query = doc! {"alias": alias.to_string() };
        match mongo.find_one_and_delete(query, None).await {
            Ok(Some(_)) => Custom(
                Status::Ok,
                json!({"status": Status::Ok.reason(), "message": "User deleted"}),
            ),
            Ok(_) => Custom(
                Status::NotFound,
                json!({"status": Status::NotFound.reason(), "message": "User already deleted"}),
            ),
            Err(_) => Custom(
                Status::InternalServerError,
                json!({"status": Status::InternalServerError.reason(), "message": "Database error"}),
            ),
        }
    } else {
        Custom(
            Status::Unauthorized,
            json!({"status":Status::Unauthorized.reason(), "message": "You can only delete your own user"}),
        )
    }
}
