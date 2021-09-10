use mongodb::bson::doc;
use mongodb::Collection;
use rocket::futures::StreamExt;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::result::{ApiResult};
use crate::api::sessions::data::PublicsessionData;
use crate::api::users::auth::token::claims::TokenClaims;
use crate::mongo::user::Session;
use crate::api::SESSION_USER_ALIAS;

/// # AUTH! `GET /api/sessions`
/// Returns all current sessions from the user
///
/// # Returns
/// ## Ok (200)
///
/// ```json
/// [{
///     "ip": String,
///     "date": String
/// },
///
/// ...]
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
/// | 500 | Couldn't connect to database |
///
/// # Example
///
/// `GET /api/sessions`
///
/// ```json
/// [{
///     "ip": "127.0.0.1",
///     "date": "2021-09-08 12:36:51.077 UTC"
/// }]
/// ```
#[get("/", format = "json")]
pub async fn get_user_sessions(
    session_collection: &State<Collection<Session>>,
    token: TokenClaims,
) ->  ApiResult<Json<Vec<PublicsessionData>>> {
    let filter = doc! { SESSION_USER_ALIAS : mongodb::bson::to_bson(token.alias()).unwrap() };
    let mut cursor = session_collection.find(filter, None).await?;

    let mut vec = Vec::new();
    while let Some(res) = cursor.next().await {
        vec.push(PublicsessionData::from_session(res?));
    }
    Ok(Json(vec))
}
