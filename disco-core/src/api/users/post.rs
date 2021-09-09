use std::collections::HashMap;

use mongodb::bson::doc;
use mongodb::Collection;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::result::ApiError;
use crate::api::result::ApiError::InternalServerError;
use crate::api::sessions::delete_all_sessions_from;
use crate::api::users::auth::token::claims::TokenClaims;
use crate::api::users::data::{UpdatePassword, UpdateUser};
use crate::mongo::user::{Email, Password, Session, User, Description};

/// # AUTH! `POST /api/users/update/password`
/// Changes the user password to another one
///
/// ```json
/// {
///     "password": String,
///     "new_password": String
/// }
/// ```
///
/// # Returns
/// ## Ok (204)
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
/// | 400 | Bad request |
/// | 404 | User doesn't exist |
/// | 500 | Couldn't connect to database |
///
/// # Example
///
/// `POST /api/users/update/password`
///
/// ## Body payload
///
/// ```
/// {
///     "password": "theoldpassword",
///     "new_password": "thenewpassword"
/// }
/// ```
///
/// ## Response (204)
#[post("/update/password", format = "json", data = "<updated>")]
pub async fn update_user_password(
    updated: Json<UpdatePassword<'_>>,
    user_collection: &State<Collection<User>>,
    session_collection: &State<Collection<Session>>,
    token: TokenClaims,
) -> Result<rocket::response::status::NoContent, ApiError> {
    let validated_document = updated.new_password.parse::<Password>()?;
    let user = crate::api::users::locate_user(token.alias(), user_collection).await?;

    match user.password().validate(updated.password) {
        Ok(true) => {
            let filter = doc! { "alias": user.alias().to_string() };
            let update_op = doc! {"$set": { "password": validated_document.password() }};
            let _response = user_collection.update_one(filter, update_op, None).await?;
            delete_all_sessions_from(user.alias(), session_collection).await?;
            Ok(rocket::response::status::NoContent)
        }
        Ok(false) => Err(ApiError::Unauthorized("Invalid password")),
        Err(_) => Err(InternalServerError("Couldn't hash password")),
    }
}
/*
#[post("/update/avatar", format = "json", data = <media>)]
pub async fn update_user_avatar(
    updated: Json<Upda>
)*/

/// # AUTH! `POST /api/users/update/`
///
/// Updates the user data with the recived data
///
/// ```json
/// {
///     "email": String,        // Optional
///     "description": String   // Optional
/// }
/// ```
///
/// # Returns
/// ## Ok (204)
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
/// | 400 | Bad request (invalid email or description) |
/// | 404 | User doesn't exist |
/// | 500 | Couldn't connect to database |
///
/// # Example
///
/// `POST /api/users/update`
///
/// ## Body payload
///
/// ```json
/// {
///     "email": "thenew@email.com"
/// }
/// ```
///
/// ## Response (204)
#[post("/update", format = "json", data = "<updated>")]
pub async fn update_user_info(
    updated: Json<UpdateUser<'_>>,
    user_collection: &State<Collection<User>>,
    token: TokenClaims,
) -> Result<rocket::response::status::NoContent, ApiError> {
    let mut dic = HashMap::new();
    if let Some(s) = updated.email {
        let _ = s.parse::<Email>()?;
        dic.insert("email", s);
    }

    if let Some(s) = updated.description {
        let _ = s.parse::<Description>()?;
        dic.insert("description", s);
    }

    // Unwrap is safe. Valid string slices
    let update_doc = doc! {
        "$set": mongodb::bson::to_document(&dic).unwrap()
    };

    let filter = doc! { "alias": token.alias().alias().to_string() };
    let res = user_collection.update_one(filter, update_doc, None).await?;

    if res.modified_count == 1 {
        Ok(rocket::response::status::NoContent)
    } else {
        Err(ApiError::NotFound("User"))
    }
}
