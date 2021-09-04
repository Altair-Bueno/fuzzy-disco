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

const TTL : i64 = 200;

// , format = "application/x-www-form-urlencoded"
#[put("/upload", data = "<file>")]
pub async fn upload(mut file: TempFile<'_>,files: &State<CacheFiles>) -> JsonResult<DictionaryResponse> {
    // TODO ttl
    let ttl = chrono::Utc::now() + chrono::Duration::seconds(TTL);
    let key =  format!("{}-{}",ttl,rand::random::<usize>());
    let filename = format!("temp/{}",key);
    if let Err(x) = file.copy_to(filename.as_str()).await {
        return Err(status::Custom(Status::InternalServerError,doc! {"message": "Couldn't store file","err":x.to_string()}.to_string()))
    }

    if let Some(_) = files.insert(key.clone(),filename) {
        files.remove(key.as_str());
        return Err(status::Custom(Status::InternalServerError,doc!{"message": "Key collision"}.to_string()));
    }

    let response = hashmap! {
        "media_key" => key,
        "ttl" =>   ttl.to_string()
    };

    Ok(Json(response))
}