use crate::api::users::auth::token::claims::TokenClaims;
use crate::mongo::sesion::Sesion;
use mongodb::Collection;
use rocket::State;
use crate::api::sesions::delete_all_sesions_from;
use crate::api::result::ApiError;

#[post("/delete", data="<body>")]
pub async fn delete_all_sessions(
    sesion_collection: &State<Collection<Sesion>>,
    token: TokenClaims,
    body:&str
) -> Result<rocket::response::status::NoContent,ApiError>{
    if !body.is_empty(){
        return Err(ApiError::BadRequest("Body must be empty"))
    } else {
        delete_all_sesions_from(token.alias(),sesion_collection)
            .await
            .map(|_|rocket::response::status::NoContent)
    }
}