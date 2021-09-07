use crate::api::result::ApiError;
use crate::api::result::ApiError::InternalServerError;
use crate::api::users::auth::token::claims::TokenClaims;
use crate::api::users::data::{UpdatePassword, UpdateUser};
use crate::mongo::user::{Password, Sesion, User, Email};
use mongodb::bson::doc;
use mongodb::Collection;
use rocket::serde::json::Json;
use rocket::State;
use std::collections::HashMap;

/// # AUTH! `PUT /api/users/update/password`
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
/// `PUT /api/users/update/password`
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
#[put("/update/password", format = "json", data = "<updated>")]
pub async fn update_user_password(
    updated: Json<UpdatePassword<'_>>,
    user_collection: &State<Collection<User>>,
    sesion_collection: &State<Collection<Sesion>>,
    token: TokenClaims,
) -> Result<rocket::response::status::NoContent, ApiError> {
    let validated_document = updated.new_password.parse::<Password>()?;
    let user = crate::api::users::locate_user(token.alias(), user_collection).await?;

    match user.password().validate(updated.password) {
        Ok(true) => {
            let filter = doc! { "alias": user.alias().to_string() };
            let update_op = doc! {"$set": { "password": validated_document.password() }};
            let _response = user_collection.update_one(filter, update_op, None).await?;
            let filter = doc! { "user_alias": user.alias().alias() };
            let _response = sesion_collection.delete_many(filter, None).await?;
            Ok(rocket::response::status::NoContent)
        }
        Ok(false) => Err(ApiError::Unauthorized("Invalid password")),
        Err(_) => Err(InternalServerError("Couldn't hash password")),
    }
}

/// # AUTH! `PUT /api/users/update/`
///
/// Updates the user data with the recived data
///
/// ```json
/// {
///     "email": String
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
/// `PUT /api/users/update`
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

#[put("/update", format = "json", data = "<updated>")]
pub async fn update_user_info(
    updated: Json<UpdateUser<'_>>,
    user_collection: &State<Collection<User>>,
    token: TokenClaims,
) -> Result<rocket::response::status::NoContent, ApiError> {
    let mut dic = HashMap::new();
    if let Some (s) = updated.email {
        let email = s.parse::<Email>()?;
        dic.insert("email",s);
    }
    // more fields if needed
    // Unwrap is safe. Valid string slices
    let update_doc = doc! {
        "$set": mongodb::bson::to_document(&dic).unwrap()
    };
    let filter = doc! { "alias": token.alias().alias().to_string() };
    user_collection.update_one(filter,update_doc,None).await?;
    Ok(rocket::response::status::NoContent)
}