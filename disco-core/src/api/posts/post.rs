use mongodb::{bson::doc, Collection};
use rocket::response::status::Created;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::State;

use crate::api::media::{claim_media_update, unclaim_media_update, is_expired};
use crate::api::posts::data::NewPostPayload;
use crate::api::result::{ApiError, ApiResult};
use crate::api::users::auth::claims::TokenClaims;
use crate::mongo::media::{Format, Media, Status};
use crate::mongo::post::Post;
use crate::api::{MEDIA_UPLOADED_BY, MEDIA_ID, MEDIA_FORMAT, MEDIA_STATUS};

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

    if is_expired(&audio) || is_expired(&photo) {
        return Err(ApiError::BadRequest("Expired file"))
    }

    let post = Post::new(title, caption, author, audio, photo, visibility);
    // Claim files
    let query = doc! {
        "$or": [
            {MEDIA_ID:post.photo(), MEDIA_FORMAT: Format::Image},
            {MEDIA_ID:post.audio(), MEDIA_FORMAT: Format::Audio}
        ],
        MEDIA_UPLOADED_BY: post.author(),
        MEDIA_STATUS: Status::Waiting
    };
    let update = claim_media_update().await;
    let update_result = media_collection.update_many(query,update,None).await?;

    if update_result.modified_count == 2 {
        // Insert post
        let insert_result = post_collection.insert_one(&post, None).await?;
        Ok(Created::new(
            format!("/api/posts/{}", insert_result.inserted_id.to_string()))
            .body(json!({
                "status":"Created",
                "message": "Post created",
                "post_id": insert_result.inserted_id.as_object_id().map(|x| x.to_string())
            }))
        )
    } else {
        // Some of the media was not claimed, rollback changes
        let update = unclaim_media_update().await;
        let query = doc! {
            "$or": [{MEDIA_ID:post.photo()},{MEDIA_ID:post.audio()}]
        };
        let _ = media_collection.update_many(query,update,None).await;
        Err(ApiError::BadRequest("The provided files did not exist or where already claimed"))
    }
}
