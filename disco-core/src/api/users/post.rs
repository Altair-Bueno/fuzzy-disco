use std::collections::HashMap;
use std::str::FromStr;

use mongodb::Client;
use mongodb::{bson::doc, Collection};
use rocket::serde::json::Json;
use rocket::State;

use crate::api::media::{claim_media_filter, claim_media_update, delete_media};
use crate::api::result::ApiError::InternalServerError;
use crate::api::result::{ApiError, ApiResult};
use crate::api::sessions::delete_all_sessions_from;
use crate::api::users::auth::claims::TokenClaims;
use crate::api::users::data::{AvatarPictureID, UpdatePassword, UpdateUser};
use crate::api::{MEDIA_ID, USER_ALIAS, USER_AVATAR, USER_PASSWORD};
use crate::mongo::media::{Format, Media};
use crate::mongo::user::{Description, Email, Password, Session, User};

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
/// # Race conditions
/// This API method does not provide protection against race conditions, meaning
/// that if the same user changes the password at the same time on multiple
/// sessions only one of them will succeed but none of them will receive any
/// kind of warning. This is due to mongodb ACID transactions limitations and
/// the intentional desire to make this server fast
///
/// # Returns
/// ## Ok (200)
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
/// | 401 | Old password doesn't match |
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
/// ## Response (200)
#[post("/update/password", format = "json", data = "<updated>")]
pub async fn update_user_password(
    updated: Json<UpdatePassword<'_>>,
    user_collection: &State<Collection<User>>,
    session_collection: &State<Collection<Session>>,
    token: TokenClaims,
) -> ApiResult<()> {
    let validated_document = updated.new_password.parse::<Password>()?;
    let user = crate::api::users::locate_user(token.alias(), user_collection).await?;

    match user.password().validate(updated.password) {
        Ok(true) => {
            let filter = doc! { USER_ALIAS: mongodb::bson::to_bson(user.alias()).unwrap() };
            let update_op = doc! {"$set": { USER_PASSWORD: validated_document.password() }};
            let _response = user_collection.update_one(filter, update_op, None).await?;
            delete_all_sessions_from(user.alias(), session_collection).await?;
            Ok(())
        }
        Ok(false) => Err(ApiError::Unauthorized("Invalid password")),
        Err(_) => Err(InternalServerError("Couldn't hash password")),
    }
}

/// # AUTH! `POST /api/users/update/avatar`
/// Deletes the old avatar image and updates it with the new file. If no other
/// file is provided, the actual image will be deleted.
///
/// > Note: mediaid == key from [crate::api::media::post::upload()]
///
/// ```json
/// {
///     "mediaid": String // Optional
/// }
/// ```
///
/// # Returns
/// ## Ok (200)
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
/// `POST /api/users/update/avatar`
///
/// ## Response (200)
#[post("/update/avatar", format = "json", data = "<updated>")]
pub async fn update_user_avatar(
    token: TokenClaims,
    updated: Json<AvatarPictureID>,
    user_collection: &State<Collection<User>>,
    media_collection: &State<Collection<Media>>,
) -> ApiResult<()> {
    let avatar_id = {
        if let Some(id) = updated.0.media_id {
            let oid = id.extract();
            // Claim media
            let filter = claim_media_filter(&oid, &Format::Image, token.alias()).await;
            let update = claim_media_update().await;
            let media = media_collection
                .find_one_and_update(filter, update, None)
                .await?
                .ok_or(ApiError::BadRequest("Media file not found"))?;
            media.id()
        } else {
            None
        }
    };
    let filter = doc! { USER_ALIAS: mongodb::bson::to_bson(token.alias()).unwrap() };
    let update = doc! {"$set": { USER_AVATAR: avatar_id }};
    let user_before = user_collection
        .find_one_and_update(filter, update ,None)
        .await?
        .ok_or(ApiError::NotFound("User"))?;
    // Delete the old media file
    let filter = doc! {MEDIA_ID: user_before.avatar() };
    let _ = media_collection.delete_one(filter,None).await?;
    if let Some(id) = user_before.avatar() {
        let _ = delete_media(&id).await;
    }
    Ok(())
}

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
/// ## Ok (200)
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
/// ## Response (200)
#[post("/update", format = "json", data = "<updated>")]
pub async fn update_user_info(
    updated: Json<UpdateUser<'_>>,
    user_collection: &State<Collection<User>>,
    token: TokenClaims,
) -> ApiResult<()> {
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
        "$set": mongodb::bson::to_bson(&dic).unwrap()
    };

    let filter = doc! { USER_ALIAS: mongodb::bson::to_bson(token.alias()).unwrap() };
    let res = user_collection.update_one(filter, update_doc, None).await?;

    if res.modified_count == 1 {
        Ok(())
    } else {
        Err(ApiError::NotFound("User"))
    }
}
