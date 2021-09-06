use crate::api::result::ApiResult;
use crate::mongo::user::{Alias, User, Password, UserError};
use mongodb::bson::doc;
use mongodb::Collection;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::State;
use rocket::serde::json::{Value, Json};
use crate::api::users::auth::token::claims::TokenClaims;
use std::collections::HashMap;
use crate::api::users::data::UpdatePassword;
use std::future::Future;

#[put("/update/password", format = "json", data="<updated>")]
pub async fn update_user_info(updated: Json<UpdatePassword<'_>>, mongo: &State<Collection<User>>, token: TokenClaims) -> Result<rocket::response::status::NoContent,Custom<Value>> {
    let validated_document = match create_update_document(&updated).await {
        Ok(elem) => elem,
        Err(x) => return Err(Custom(Status::BadRequest, json!({
                    "status": Status::BadRequest.reason(),
                    "message": x
                })))
    };
    let user = crate::api::users::get::locate_user(token.alias(), mongo).await?;
    match user.password().validate(updated.password) {
        Ok(true) =>{
            let filter = doc! { "alias": user.alias().to_string() };
            let update_op = doc! {"$set": validated_document};
            match mongo.update_one(filter, update_op, None).await {
                Ok(_) => Ok(rocket::response::status::NoContent),
                Err(_) => {
                    Err(Custom(Status::InternalServerError,
                                   json!({
                                        "status": Status::InternalServerError.reason(),
                                        "message": "Couldn't connect to database"
                                    })))
                }
            }
        },
        Ok(false) => Err(Custom(Status::BadRequest, json!({
                    "status": Status::BadRequest.reason(),
                    "message": "Password incorrect"
                }))),
        Err(_) => Err(Custom(Status::InternalServerError, json!({
                    "status": Status::InternalServerError.reason(),
                    "message": "Couldn't verify password"
                })))
    }
}

async fn create_update_document(updated:&Json<UpdatePassword<'_>>) -> Result<mongodb::bson::Document, UserError> {
    let mut document = HashMap::new();
    if let Some(password) = updated.new_password.map(|x|x.parse::<Password>()) {
        let password = password?;
        document.insert("password", password.to_string());
    }
    Ok(mongodb::bson::to_document(&document).unwrap())
}