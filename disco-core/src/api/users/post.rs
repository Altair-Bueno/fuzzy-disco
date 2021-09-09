use std::collections::HashMap;

use mongodb::{Client};
use rocket::serde::json::Json;
use rocket::State;

use crate::api::result::ApiError;
use crate::api::result::ApiError::InternalServerError;
use crate::api::sessions::delete_all_sessions_from;
use crate::api::users::auth::token::claims::TokenClaims;
use crate::api::users::data::{UpdatePassword, UpdateUser, AvatarPictureID};
use crate::mongo::user::{Email, Password, Session, User, Description};
use crate::mongo::media::{Media, Format};
use std::str::FromStr;
use rocket::response::status::NoContent;

use mongodb::{
    bson::{doc, Document},
    options::{Acknowledgment, ReadConcern, TransactionOptions, WriteConcern},
    Collection,
};
use crate::api::media::{claim_media_filter, claim_media_update};

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
            let filter = doc! { "alias": mongodb::bson::to_bson(user.alias()).unwrap() };
            let update_op = doc! {"$set": { "password": validated_document.password() }};
            let _response = user_collection.update_one(filter, update_op, None).await?;
            delete_all_sessions_from(user.alias(), session_collection).await?;
            Ok(rocket::response::status::NoContent)
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
/// `POST /api/users/update/avatar`
///
/// ## Response (204)
#[post("/update/avatar", format = "json", data = "<updated>")]
pub async fn update_user_avatar (
    token: TokenClaims,
    updated: Json<AvatarPictureID<'_>>,
    user_collection: &State<Collection<User>>,
    media_collection: &State<Collection<Media>>,
    mongo_client: &State<Client>
)-> Result<rocket::response::status::NoContent,ApiError> {
    // TODO if avatar picture id has already a photo
    let oid = mongodb::bson::oid::ObjectId::from_str(updated.mediaid)?;
    let mut transaction_session = mongo_client.start_session(None).await?;
    let options = TransactionOptions::builder()
        .read_concern(ReadConcern::majority())
        .write_concern(WriteConcern::builder().w(Acknowledgment::Majority).build())
        .build();
    transaction_session.start_transaction(options).await?;
    // Claim media
    let filter = claim_media_filter(&oid,&Format::Image,token.alias()).await;
    let update = claim_media_update().await;
    let media = media_collection.find_one_and_update(filter,update,None).await?
        .ok_or(ApiError::BadRequest("Media file not found"))?;
    // Update user
    let filter = doc! {"alias": mongodb::bson::to_bson(token.alias()).unwrap() };
    let update = doc! {"$set": {"avatar": media.id() }};
    let result = user_collection.update_one(filter,update,None).await?;

    if result.modified_count == 1 {
        transaction_session.commit_transaction().await?;
        Ok(NoContent)
    } else {
        Err(ApiError::NotFound("User"))
    }
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
        "$set": mongodb::bson::to_bson(&dic).unwrap()
    };

    let filter = doc! { "alias": mongodb::bson::to_bson(token.alias()).unwrap() };
    let res = user_collection.update_one(filter, update_doc, None).await?;

    if res.modified_count == 1 {
        Ok(rocket::response::status::NoContent)
    } else {
        Err(ApiError::NotFound("User"))
    }
}
