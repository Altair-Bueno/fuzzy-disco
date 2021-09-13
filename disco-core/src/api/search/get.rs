use crate::api::result::ApiResult;
use crate::api::search::data::SearchQuery;
use rocket::State;
use mongodb::Collection;
use crate::mongo::user::User;
use crate::mongo::post::Post;
use rocket::serde::json::Json;

// TODO check if it works and add documentation
#[get("/?<s>&<drop>&<get>&<date>")]
pub async fn search(
    s:&str,
    drop:usize,
    get:u8,
    date:&str,
    user_collection: &State<Collection<User>>,
    posts_collection: &State<Collection<Post>>,
) -> ApiResult<Json<Vec<String>>> {
    todo!()
}