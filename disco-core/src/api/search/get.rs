use crate::api::result::ApiResult;
use crate::api::search::data::SearchQuery;
use rocket::State;
use mongodb::Collection;
use crate::mongo::user::User;
use crate::mongo::post::Post;

// TODO check if it works and add documentation
#[get("/?<search>")]
pub async fn search_posts(
    search:SearchQuery<'_>,
    user_collection: &State<Collection<User>>,
    posts_collection: &State<Collection<Post>>,
) -> ApiResult<()> {
    todo!()
}