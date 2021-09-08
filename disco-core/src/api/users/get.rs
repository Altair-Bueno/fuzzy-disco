use mongodb::bson::doc;
use mongodb::Collection;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::State;

use crate::api::result::ApiError;
use crate::api::users::auth::token::claims::TokenClaims;
use crate::mongo::user::{Alias, User};

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
pub async fn get_user_info(
    alias: &str,
    mongo: &State<Collection<User>>,
) -> Result<Custom<Value>, ApiError> {
    let alias = alias.parse::<Alias>()?;
    let user = crate::api::users::locate_user(&alias, mongo).await?;
    Ok(Custom(
        Status::Ok,
        json!({
                "alias": user.alias(),
                "posts": user.posts(),
            }
        ),
    ))
}

/// # AUTH! `GET /api/users`
/// Returns the **private** information stored about the user. This includes
/// everything except the hashed password
///
/// # Returns
/// ## Ok (200)
///
/// ```json
/// {
///     "alias": String,
///     "posts": [String],
///     "email": String,
///     "creation_date": Date
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
///
/// # Example
///
/// `GET /api/users/altair-bueno`
///
/// ```json
/// {
///   "alias": "helloworld",
///   "posts": [
///   ],
///   "email": "e@hello.es",
///   "creation_date": "2021-09-06 16:13:02.797 UTC"
/// }
/// ```
#[get("/")]
pub async fn get_full_user_info(
    mongo: &State<Collection<User>>,
    token: TokenClaims,
) -> Result<Custom<Value>, ApiError> {
    let user = crate::api::users::locate_user(token.alias(), mongo).await?;
    Ok(Custom(
        Status::Ok,
        json!({
            "alias": user.alias(),
            "posts": user.posts(),
            "email": user.email(),
            "creation_date": user.creation_date().to_string()
        }),
    ))
}
