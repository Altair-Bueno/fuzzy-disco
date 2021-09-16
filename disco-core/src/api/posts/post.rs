use mongodb::{bson::doc, Collection};
use rocket::response::status::Created;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::State;

use crate::api::media::{claim_media_filter, claim_media_update, delete_media};
use crate::api::posts::data::NewPostPayload;
use crate::api::result::{ApiError, ApiResult};
use crate::api::users::auth::claims::TokenClaims;
use crate::mongo::media::{Format, Media};
use crate::mongo::post::Post;
use crate::api::POSTS_ID;

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
#[post("/new", format = "json", data = "<payload>")]
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
    let visibility = payload
        .visibility
        .parse()
        .map_err(|_| ApiError::BadRequest("Invalid visibility"))?;
    let post = Post::new(title, caption, author, audio, photo, visibility);
    // Claim image
    let claim_image = claim_media_filter(&post.photo(), &Format::Image, post.author()).await;
    let update = claim_media_update().await;
    let update_result = media_collection
        .update_one(claim_image, update, None)
        .await?;
    if update_result.modified_count != 1 {
        return Err(ApiError::NotFound("Photo"));
    }
    // Claim audio
    let claim_audio = claim_media_filter(&post.audio(), &Format::Audio, post.author()).await;
    let update = claim_media_update().await;
    let update_result = media_collection
        .update_one(claim_audio, update, None)
        .await?;
    if update_result.modified_count != 1 {
        // delete the already claimed media
        let _ = delete_media(&post.photo()).await;
        let _ = media_collection.delete_one(doc! {POSTS_ID:post.photo()},None).await;
        return Err(ApiError::NotFound("Audio"));
    }
    // Insert post
    let insert_result = post_collection.insert_one(&post, None).await?;
    Ok(Created::new(
        format!("/api/posts/{}", insert_result.inserted_id.to_string()))
        .body(json!({"status":"Created","message": "Post created", "post_id": insert_result.inserted_id.as_object_id().map(|x| x.to_string())}))
    )
}
