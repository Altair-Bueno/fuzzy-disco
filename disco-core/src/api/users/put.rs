use crate::api::result::ApiResult;
use crate::mongo::user::{Alias, User, Password, UserError, Sesion};
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
use rocket::http::ext::IntoCollection;

#[put("/update/password", format = "json", data="<updated>")]
pub async fn update_user_password(
    updated: Json<UpdatePassword<'_>>,
    user_collection: &State<Collection<User>>,
    sesion_collection: &State<Collection<Sesion>>,
    token: TokenClaims
) -> Result<rocket::response::status::NoContent,Custom<Value>> {
    let validated_document = match updated.new_password.parse::<Password>() {
        Ok(x) => x,
        Err(x)=> return Err(Custom(Status::BadRequest, json!({
                    "status": Status::BadRequest.reason(),
                    "message": x
                })))
    };
    let user = crate::api::users::get::locate_user(token.alias(), user_collection).await?;
    match user.password().validate(updated.password) {
        Ok(true) =>{
            let filter = doc! { "alias": user.alias().to_string() };
            let update_op = doc! {"$set": { "password": validated_document.password() }};
            match user_collection.update_one(filter, update_op, None).await {
                Ok(_) => {
                    let filter = doc! { "user_alias": user.alias().alias() };
                    let result = sesion_collection.delete_many(filter,None).await;
                    match result {
                        Ok(_)=> Ok(rocket::response::status::NoContent),
                        Err(_) =>Err(Custom(Status::InternalServerError,
                                            json!({
                                        "status": Status::InternalServerError.reason(),
                                        "message": "Couldn't connect to database"
                                    })))
                    }
                },
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
