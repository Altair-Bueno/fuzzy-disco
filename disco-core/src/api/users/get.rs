use crate::api::result::ApiResult;
use crate::mongo::user::{Alias, User, UserError};
use mongodb::bson::doc;
use mongodb::Collection;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::State;

/// # `GET /api/users/<alias>`
/// Returns the public information avaliable for the given user
///
/// # Returns
/// ## Ok (200)
///
/// ```json
/// {
///     "alias": String,
///     "posts": [String],
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
/// | 400 | `alias` isn't correctly formated |
/// | 404 | User doesn't exist |
/// | 500 | Couldn't connect to database |
///
/// # Example
///
/// `GET /api/users/altair-bueno`
///
/// ```json
/// {
///  "alias": "altair-bueno",
///  "posts": [
///     "6132137e6c2cc66344ef2a88"
///  ]
///}
/// ```
#[get("/<alias>")]
pub async fn get_user_info(alias: &str, mongo: &State<Collection<User>>) -> ApiResult {
    let alias = match alias.parse::<Alias>() {
        Ok(x) => x,
        Err(_) => {
            return Custom(
                Status::BadRequest,
                json!({"status":Status::BadRequest.reason(),"message": "Invalid alias"}),
            )
        }
    };
    let result = mongo
        .find_one(doc! {"alias": alias.to_string() }, None)
        .await;
    match result {
        Ok(Some(x)) => Custom(
            Status::Ok,
            json!({
                "alias": x.alias(),
                "posts": x.posts(),
            }),
        ),
        Ok(None) => Custom(
            Status::NotFound,
            json!({"status":Status::NotFound.reason(),"message": "User doesn't exist"}),
        ),
        Err(_) => Custom(
            Status::InternalServerError,
            json!({"status": Status::InternalServerError.reason(),"message": "Couldn't connect to database"}),
        ),
    }
}
