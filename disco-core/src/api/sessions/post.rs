use mongodb::Collection;
use rocket::State;

use crate::api::result::{ApiError, ApiResult};
use crate::api::sessions::delete_all_sessions_from;
use crate::api::users::auth::token::claims::TokenClaims;
use crate::mongo::session::Session;

/// # AUTH! `POST /api/sessions/delete`
///
/// Deletes all sessions from the current user, included the current one. Can be
/// used to log out on all browsers, for example
///
/// > Note: This is a no body post request, with no body response
///
/// # Returns
/// ## Ok (200)
///
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
/// | 400 | Body is not empty |
/// | 500 | Couldn't connect to database |
///
/// # Example
///
/// `POST /api/sessions/delete`
#[post("/delete", data = "<body>")]
pub async fn delete_all_sessions(
    session_collection: &State<Collection<Session>>,
    token: TokenClaims,
    body: &str,
) -> ApiResult<()> {
    if !body.is_empty() {
        Err(ApiError::BadRequest("Body must be empty"))
    } else {
        delete_all_sessions_from(token.alias(), session_collection)
            .await
    }
}
