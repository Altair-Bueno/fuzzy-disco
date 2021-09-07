use crate::api::result::ApiError;
use crate::api::result::ApiError::InternalServerError;
use crate::api::users::auth::token::claims::TokenClaims;
use crate::api::users::data::UpdatePassword;
use crate::mongo::user::{Password, Sesion, User};
use mongodb::bson::doc;
use mongodb::Collection;
use rocket::serde::json::Json;
use rocket::State;

#[put("/update/password", format = "json", data = "<updated>")]
pub async fn update_user_password(
    updated: Json<UpdatePassword<'_>>,
    user_collection: &State<Collection<User>>,
    sesion_collection: &State<Collection<Sesion>>,
    token: TokenClaims,
) -> Result<rocket::response::status::NoContent, ApiError> {
    let validated_document = updated.new_password.parse::<Password>()?;
    let user = crate::api::users::get::locate_user(token.alias(), user_collection).await?;

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
