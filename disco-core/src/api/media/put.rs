use chrono::{DateTime, Utc};
use dashmap::mapref::entry::Entry;
use maplit::hashmap;
use mongodb::bson::doc;
use rand::random;
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::tokio::sync::mpsc::Sender;
use rocket::State;

use crate::api::result::{DictionaryResponse, JsonResult};
use crate::CacheFiles;

#[cfg(debug_assertions)]
const TTL: u64 = 10;
#[cfg(not(debug_assertions))]
const TTL: u64 = 60;

// , format = "application/x-www-form-urlencoded"
#[put("/upload", data = "<file>")]
pub async fn upload(
    file: TempFile<'_>,
    cache_files: &State<CacheFiles>,
    gc: &State<Sender<String>>,
) -> JsonResult<DictionaryResponse> {
    let recived_at = Utc::now();
    let key = match temporal_store(recived_at, file, cache_files, gc).await {
        Ok(key) => key,
        Err(err) => {
            return Err(status::Custom(
                Status::InternalServerError,
                doc! {"message": err.to_string()}.to_string(),
            ))
        }
    };
    let response = hashmap! {
        "key" => key,
        "TTL" => TTL.to_string(),
    };

    Ok(Json(response))
}

pub async fn temporal_store(
    recived_date: DateTime<Utc>,
    mut file: TempFile<'_>,
    cache_files: &State<CacheFiles>,
    gc: &State<Sender<String>>,
) -> std::io::Result<String> {
    // Find unike key
    let (key, path) = {
        loop {
            let key = format!("{}-{}", recived_date, random::<usize>());
            let filename = format!("temp/{}", key);
            if let Entry::Vacant(x) = cache_files.entry(key.clone()) {
                x.insert(filename.clone());
                break (key, filename);
            }
        }
    };
    // Copy the temporal file
    file.copy_to(&path).await?;

    // Set up GC
    let gc_clone = (*gc).clone();
    let cache_files_clone = (*cache_files).clone();
    let key_clone = key.clone();
    rocket::tokio::spawn(async move {
        rocket::tokio::time::sleep(rocket::tokio::time::Duration::new(TTL, 0)).await;
        let entry = cache_files_clone.remove(&key_clone);
        if let Some((_, expired)) = entry {
            let _ = gc_clone.send(expired).await;
        }
    });

    Ok(key)
}
