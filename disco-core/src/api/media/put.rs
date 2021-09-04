use rocket::fs::TempFile;
use rocket::{State, Request};
use crate::CacheFiles;
use crate::api::result::{JsonResult, DictionaryResponse};
use std::collections::HashMap;
use rocket::response::status;
use rocket::http::Status;
use mongodb::bson::doc;
use maplit::hashmap;
use rocket::serde::json::Json;
use std::sync::{Arc, Mutex};
use rocket::tokio::sync::mpsc::Sender;

const TTL : u64 = 5;

// , format = "application/x-www-form-urlencoded"
#[put("/upload", data = "<file>")]
pub async fn upload(mut file: TempFile<'_>,cache_files: &State<CacheFiles>, gc : &State<Sender<String>>) -> JsonResult<DictionaryResponse> {
    let ttl = chrono::Utc::now();
    let key =  format!("{}-{}",ttl,rand::random::<usize>());
    let location = format!("temp/{}", key);

    if let Err(x) = file.copy_to(location.as_str()).await {
        return Err(status::Custom(Status::InternalServerError,doc! {"message": "Couldn't store file","err":x.to_string()}.to_string()))
    }

    if let Some(_) = cache_files.insert(key.clone(), location) {
        cache_files.remove(key.as_str());
        return Err(status::Custom(Status::InternalServerError,doc!{"message": "Key collision"}.to_string()));
    }

    let response = hashmap! {
        "media_key" => key.clone(),
        "ttl" => TTL.to_string()
    };

    let gc_clone = (*gc).clone();
    let cache_files_clone = (*cache_files).clone();
    rocket::tokio::spawn(async move {
        rocket::tokio::time::sleep(rocket::tokio::time::Duration::new(TTL,0)).await;
        let entry = cache_files_clone.remove(&key);
        if let Some((_,expired)) = entry {
            let _ = gc_clone.send(expired).await;
        }
    });

    Ok(Json(response))
}

pub async fn temporal_store (
    mut file : TempFile<'_>,
    cache_files:&State<CacheFiles>,
    gc:&State<Sender<String>>
) -> std::io::Result<String> {
todo!()
}