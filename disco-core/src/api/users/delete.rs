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
use crate::api::sesions::delete_all_sesions_from;

/// # AUTH! `DELETE /api/users`
/// Deletes the current authenticated user from the database
///
/// # Returns
/// ## Ok (200)
///
/// ```json
/// {
///     "status": "Ok",
///     "message": "User deleted"
/// }
/// ```
///
/// ## Err
/// ```json
/// {
///     "status": String,
///     "message": String
/// }
/// ```
///
/// | Code | Description |
/// | -----| ----------- |
/// | 404 | User doesn't exist |
/// | 500 | Couldn't connect to database |
/// ```
#[delete("/")]
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
           delete_all_sesions_from(bearer_token_alias,sesion_collection).await?;
            Ok(Custom(
                Status::Ok,
                json!({"status": Status::Ok.reason(), "message": "User deleted"}),
            ))
        }
        None => Err(ApiError::NotFound("User")),
    }
}
