use crate::mongo::user::{User, Alias};
use crate::api::result::ApiError;
use crate::mongo::sesion::Sesion;
use mongodb::Collection;
use rocket::State;
use mongodb::bson::doc;

pub mod post;
pub mod get;
pub mod data;

pub async fn delete_all_sesions_from(user_alias:&Alias,session_collection: &State<Collection<Sesion>>) -> Result<(),ApiError>{
    let filter = doc! { "user_alias": user_alias.to_string() };
    session_collection.delete_many(filter, None).await
        .map(|_| ())
        .map_err(|x| ApiError::DatabaseError(x))
}