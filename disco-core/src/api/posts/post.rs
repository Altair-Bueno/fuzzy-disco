use crate::api::users::auth::token::claims::TokenClaims;
use crate::api::result::{ApiResult, ApiError};
use rocket::serde::json::{Json, Value};
use crate::api::posts::data::NewPostPayload;
use rocket::State;
use crate::mongo::post::Post;
use crate::mongo::media::{Media, Format};
use mongodb::{
    Client,
    bson::doc,
    options::{Acknowledgment, ReadConcern, TransactionOptions, WriteConcern},
    Collection,
};
use crate::mongo::IntoDocument;
use crate::api::media::{claim_media_filter, claim_media_update};
use rocket::response::status::Created;
use rocket::serde::json::serde_json::json;


#[post("/new",format = "json", data="<payload>")]
pub async fn new_post(
    token: TokenClaims,
    payload: Json<NewPostPayload<'_>>,
    post_collection: &State<Collection<Post>>,
    media_collection: &State<Collection<Media>>,
    mongo_client: &State<Client>
) -> ApiResult<Created<Value>> {
    let title = payload.title.parse()?;
    let caption = payload.caption.parse()?;
    let author = token.alias().clone();
    let audio = payload.audio.parse()?;
    let photo = payload.photo.parse()?;
    let visibility = payload.visibility.parse().map_err(|_| ApiError::BadRequest("Invalid visibility"))?;
    let post = Post::new(title,caption,author,audio,photo,visibility);

    // Init transaction
    let mut transaction_session = mongo_client.start_session(None).await?;
    let options = TransactionOptions::builder()
        .read_concern(ReadConcern::majority())
        .write_concern(WriteConcern::builder().w(Acknowledgment::Majority).build())
        .build();
    transaction_session.start_transaction(options).await?;

    // Claim image
    let claim_image = claim_media_filter(&post.photo(),&Format::Image,post.author()).await;
    let update = claim_media_update().await;
    let update_result = media_collection.update_one_with_session(claim_image,update,None,&mut transaction_session).await?;
    if update_result.modified_count != 1 {
        return Err(ApiError::NotFound("Photo"));
    }
    // Claim audio
    let claim_audio = claim_media_filter(&post.audio(),&Format::Audio,post.author()).await;
    let update = claim_media_update().await;
    let update_result = media_collection.update_one_with_session(claim_audio,update,None,&mut transaction_session).await?;
    if update_result.modified_count != 1 {
        return Err(ApiError::NotFound("Audio"));
    }
    // Insert post
    let insert_result = post_collection.insert_one_with_session(&post,None, &mut transaction_session).await?;

    transaction_session.commit_transaction().await?;

    Ok(Created::new(
        format!("/api/posts/{}",insert_result.inserted_id.to_string()))
        .body(json!({"status":"Created","message": "Post created"}))
    )
}