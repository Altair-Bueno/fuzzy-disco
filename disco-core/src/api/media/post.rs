use mongodb::bson::doc;
use mongodb::Collection;
use rocket::fs::TempFile;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::State;

use crate::api::media::oid_to_folder;
use crate::api::result::ApiError;
use crate::api::users::auth::token::claims::TokenClaims;
use crate::mongo::media::{Format, Media};

#[cfg(debug_assertions)]
const TTL: u64 = 3600;

#[cfg(not(debug_assertions))]
const TTL: u64 = 60;

/// # AUTH! `POST /api/media/upload`
///
/// Uploads the file to the server and stores it temporarly. The file **must**
/// be claimed before the Time To Live expires, otherwise the server will delete
/// the file. You can claim a file by using it as an *user avatar* or *post*
///
/// > Note: The key attribute on the response is the media ID. Don't loose it!!
///
/// # Supported files:
///
/// ## Image
/// - jpeg
/// - png
///
/// ## Audio
/// - mp3
///
/// # Response
///
/// ## Ok
/// ```json
/// {
///     "key": String,
///     "TTL": u64          // Seconds
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
/// | 400 | Invalid file type |
/// | 404 | User doesn't exist |
/// | 500 | Couldn't connect to database. Couldn't store file|
///
/// # Example
///
/// `POST /api/media/upload`
///
/// ```json
/// {
///     "key": "88ea329a",
///     "TTL": 60
/// }
/// ```
#[post("/upload", data = "<file>")]
pub async fn upload(
    token: TokenClaims,
    mut file: TempFile<'_>,
    mongo: &State<Collection<Media>>,
) -> Result<Json<Value>, ApiError> {
    // TODO More variants
    // inspect file
    let file_type: Format = file
        .path()
        .ok_or(ApiError::InternalServerError("Couldn't inspect file"))
        .map(infer::get_from_path)??
        .ok_or(ApiError::BadRequest("Unknown file format"))
        .map(|x| x.mime_type().parse())??;

    // insert document
    let media = Media::new(token.alias().clone(), file_type);
    let inserted = mongo.insert_one(media, None).await?;
    // Unwrap is safe. If the document has been inserted, it contains an oid
    let oid = inserted.inserted_id.as_object_id().unwrap();
    // copy to folder
    let folder = oid_to_folder(&oid);
    let path = format!("{}/{}.blob", folder, oid);
    let _ = rocket::tokio::fs::create_dir_all(&folder).await;
    file.copy_to(&path).await?;
    let response = json!({ "key" : oid.to_string(), "TTL" : TTL });
    timed_gc_routine(oid, path, (*mongo).clone()).await;

    Ok(Json(response))
}

/// Sets up a timed gc for a temporal file using its key. If the key is still
/// present on the CacheFiles index, it will remove the entry and send it to
/// the garbage collector routine
///
/// > NOTE: Although it is called *garbage collector*, it is **not** related to
/// > memory management. This GC is used for scheduling file removals
async fn timed_gc_routine(
    oid: mongodb::bson::oid::ObjectId,
    path: String,
    collection: Collection<Media>,
) {
    rocket::tokio::spawn(async move {
        rocket::tokio::time::sleep(rocket::tokio::time::Duration::new(TTL, 0)).await;
        let result = collection.delete_one(doc! {"_id": oid}, None).await;
        match result {
            Ok(x) if x.deleted_count == 1 => {
                #[cfg(debug_assertions)]
                println!("[GC]: Deleting {}", oid);
                let _ = rocket::tokio::fs::remove_file(path).await;
            }
            _ => {}
        }
    });
}
