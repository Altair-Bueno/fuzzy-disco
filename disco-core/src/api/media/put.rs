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

const TTL : u64 = 5;

// , format = "application/x-www-form-urlencoded"
#[put("/upload", data = "<file>")]
pub async fn upload(mut file: TempFile<'_>,files: &State<CacheFiles>, gc : &State<rocket::tokio::sync::mpsc::Sender<String>>) -> JsonResult<DictionaryResponse> {
    let ttl = chrono::Utc::now();
    let key =  format!("{}-{}",ttl,rand::random::<usize>());
    let location = format!("temp/{}", key);

    if let Err(x) = file.copy_to(location.as_str()).await {
        return Err(status::Custom(Status::InternalServerError,doc! {"message": "Couldn't store file","err":x.to_string()}.to_string()))
    }

    let guard = Arc::new(Mutex::new(Some(location)));
    if let Some(_) = files.insert(key.clone(),guard.clone()) {
        files.remove(key.as_str());
        return Err(status::Custom(Status::InternalServerError,doc!{"message": "Key collision"}.to_string()));
    }

    let response = hashmap! {
        "media_key" => key.clone(),
        "ttl" => TTL.to_string()
    };

    let gc_clone = (*gc).clone();
    rocket::tokio::spawn(async move {
        rocket::tokio::time::sleep(rocket::tokio::time::Duration::new(TTL,0)).await;
        let expired = if let Ok(mut lock) = guard.lock() {
            lock.take()
        } else {
            None
        };
        if let Some(x) = expired {
            let _ = gc_clone.send(x).await;
        }
    });

    Ok(Json(response))
}