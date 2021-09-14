use crate::api::users::auth::claims::TokenClaims;
use crate::api::result::{ApiResult, ApiError};
use rocket::serde::json::{Json, Value};
use crate::api::posts::data::{NewPostPayload};
use rocket::State;
use crate::mongo::post::Post;
use crate::mongo::media::{Media, Format};
use mongodb::{
    bson::doc,
    Collection,
};
use crate::api::media::{claim_media_filter, claim_media_update};
use rocket::response::status::Created;
use rocket::serde::json::serde_json::json;

/// #  AUTH! `POST /api/posts/new`
/// Creates a new post. A post must contain the following fields:
///
/// ```json
/// {
///     "title": String,
///     "caption": String,
///     "audio": String,
///     "photo": String,
///     "visibility": Visibility
/// }
/// ```
///
/// `audio` and `photo` must be two valid files pending to be claimed. Calling
/// this route with claimed media keys will result on `NotFound`
///
/// # Returns
/// ## Ok (201)
///
/// ```json
/// {
///     "status": "Created",
///     "message": "Post created",
///     "post_id": String
/// }
/// ```
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
/// | 404 | Media not found |
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
///     "title" "Summer",
///     "caption": "Summer holidays",
///     "audio": "sd8df8293",
///     "photo": "90s80<393",
///     "visibility": "Public"
/// }
/// ```
///
/// ## Response (201)
/// ```json
/// {
///     "status": "Created",
///     "message": "Post created",
///     "post_id": "a89d823nc890"
/// }
/// ```
#[post("/new",format = "json", data="<payload>")]
pub async fn new_post(
    token: TokenClaims,
    payload: Json<NewPostPayload<'_>>,
    post_collection: &State<Collection<Post>>,
    media_collection: &State<Collection<Media>>,
    // mongo_client: &State<Client>
) -> ApiResult<Created<Value>> {
    let title = payload.title.parse()?;
    let caption = payload.caption.parse()?;
    let author = token.alias().clone();
    let audio = payload.audio.parse()?;
    let photo = payload.photo.parse()?;
    let visibility = payload.visibility.parse().map_err(|_| ApiError::BadRequest("Invalid visibility"))?;
    let post = Post::new(title,caption,author,audio,photo,visibility);
/*  Transactions are not supported on single instances of mongodb
    TODO: Acid operations
    // Init transaction
    let mut transaction_session = mongo_client.start_session(None).await?;
    let options = TransactionOptions::builder()
        .read_concern(ReadConcern::majority())
        .write_concern(WriteConcern::builder().w(Acknowledgment::Majority).build())
        .build();
    transaction_session.start_transaction(options).await?;
*/
    // Claim image
    let claim_image = claim_media_filter(&post.photo(),&Format::Image,post.author()).await;
    let update = claim_media_update().await;
    let update_result = media_collection.update_one(claim_image,update,None).await?;
    if update_result.modified_count != 1 {
        return Err(ApiError::NotFound("Photo"));
    }
    // Claim audio
    let claim_audio = claim_media_filter(&post.audio(),&Format::Audio,post.author()).await;
    let update = claim_media_update().await;
    let update_result = media_collection.update_one(claim_audio,update,None).await?;
    if update_result.modified_count != 1 {
        return Err(ApiError::NotFound("Audio"));
    }
    // Insert post
    let insert_result = post_collection.insert_one(&post,None).await?;
    Ok(Created::new(
        format!("/api/posts/{}",insert_result.inserted_id.to_string()))
        .body(json!({"status":"Created","message": "Post created", "post_id": insert_result.inserted_id.as_object_id().map(|x| x.to_string())}))
    )
}