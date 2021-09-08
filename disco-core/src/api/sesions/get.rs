use crate::api::users::auth::token::claims::TokenClaims;
use rocket::serde::json::Json;
use crate::mongo::user::Sesion;
use mongodb::Collection;
use rocket::State;
use mongodb::bson::doc;
use rocket::futures::StreamExt;
use crate::api::result::ApiError;
use std::collections::HashMap;
use crate::api::sesions::data::PublicSesionData;

#[get("/", format = "json")]
pub async fn get_user_sesions(session_collection: &State<Collection<Sesion>>,token: TokenClaims) -> Result<Json<Vec<PublicSesionData>>, ApiError> {
    let filter = doc! { "user_alias" : token.alias().to_string() };
    let mut cursor = session_collection.find(filter, None).await?;

    let mut vec = Vec::new();
    while let Some(res) = cursor.next().await {
        vec.push(PublicSesionData::from_sesion(res?));
    }
    Ok(Json(vec))
}