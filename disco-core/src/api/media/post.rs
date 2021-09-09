use mongodb::bson::doc;
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::serde_json::json;
use rocket::State;

use crate::api::result::{ApiError};
use rocket::serde::json::Value;
use crate::api::users::auth::token::claims::TokenClaims;
use crate::mongo::media::{Format, Media};
use mongodb::Collection;

const TTL: u64 = 60;

// TODO variants for png,jpg and mp3
// , format = "application/x-www-form-urlencoded"
#[post("/upload", data = "<file>")]
pub async fn upload(
    token: TokenClaims,
    mut file: TempFile<'_>,
    mongo: &State<Collection<Media>>,
) -> Result<status::Custom<Value>, ApiError> {
    // inspect file
    let file_type : Format = file.path()
        .ok_or(ApiError::InternalServerError("Couldn't inspect file"))
        .map(|x| infer::get_from_path(x))??
        .ok_or(ApiError::BadRequest("Unknown file format"))
        .map(|x| x.mime_type().parse())??;

    // insert document
    let media = Media::new(token.alias().clone(),file_type);
    let inserted = mongo.insert_one(media, None).await?;
    // Unwrap is safe. If the document has been inserted, it contains an oid
    let oid = inserted.inserted_id.as_object_id().unwrap();
    // copy to folder
    let folder = oid_to_folder(&oid);
    let path = format!("{}/{}.blob",folder,oid);
    rocket::tokio::fs::create_dir_all(&folder).await?;
    println!("path created");
    file.copy_to(&path).await?;
    let response = json!({ "key" : oid.to_string(), "TTL" : TTL });
    timed_gc(oid,path,(*mongo).clone()).await;
    Ok(status::Custom(Status::Ok, response))
}

/// Sets up a timed gc for a temporal file using its key. If the key is still
/// present on the CacheFiles index, it will remove the entry and send it to
/// the garbage collector routine
///
/// > NOTE: Although it is called *garbage collector*, it is **not** related to
/// > memory management. This GC is used for scheduling file removals
async fn timed_gc (
    oid:mongodb::bson::oid::ObjectId,
    path: String,
    collection: Collection<Media>
) {
    rocket::tokio::spawn(async move {
        rocket::tokio::time::sleep(rocket::tokio::time::Duration::new(TTL, 0)).await;
        let result = collection.delete_one(doc! {"_id": oid},None).await;
        match result {
            Ok(x) if x.deleted_count == 1 => {
                #[cfg(debug_assertions)]
                println!("[GC]: Deleting {}", oid);
                let _ = rocket::tokio::fs::remove_file(path).await;
            },
            _=>{}
        }
    });
}

pub fn oid_to_path(oid:&mongodb::bson::oid::ObjectId) -> String {
    format!("{}/{}.blob",oid_to_folder(&oid),oid)
}

pub fn oid_to_folder(oid:&mongodb::bson::oid::ObjectId) -> String {
    oid
        .bytes()
        .iter()
        .fold("media/".to_string(),|acc,n| format!("{}/{}",acc,n))
}
