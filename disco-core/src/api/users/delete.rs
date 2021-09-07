use crate::api::result::ApiError;
use crate::api::users::auth::token::claims::TokenClaims;
use crate::mongo::sesion::Sesion;
use crate::mongo::user::User;
use mongodb::bson::doc;
use mongodb::Collection;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::State;

#[delete("/delete")]
pub async fn delete_user(
    token: TokenClaims,
    mongo: &State<Collection<User>>,
    sesion_collection: &State<Collection<Sesion>>,
) -> Result<Custom<Value>, ApiError> {
    let bearer_token_alias = token.alias();
    let query = doc! {"alias": bearer_token_alias.to_string() };
    match mongo.find_one_and_delete(query, None).await? {
        Some(_) => {
            // Delete all user sesions
            let filter = doc! { "user_alias": bearer_token_alias.to_string() };
            let _response = sesion_collection.delete_many(filter, None).await?;
            Ok(Custom(
                Status::Ok,
                json!({"status": Status::Ok.reason(), "message": "User deleted"}),
            ))
        }
        None => Err(ApiError::NotFound("User")),
    }
}
