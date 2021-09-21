use std::path::PathBuf;
use rocket::fs::NamedFile;
use rocket::response::content::Json;
use rocket::response::status::NotFound;
use serde_json::Value;

/// Matches any invalid API path and returns a customized error message
#[get("/api/<path..>", rank = 12)]
pub async fn api_bad_request(path:PathBuf)-> NotFound<Json<Value>>{
    let body = serde_json::json!({
        "status": "Not Found",
        "message": format!("/api/{} is not a valid API path", path.to_str().unwrap_or("[Unknown]"))
    });
    NotFound(Json(body))
}

/// Matches any path. Because Vue.js uses a custom router for the browser we
/// need to match any remaining path and send the index.html
#[get("/<_..>", rank = 13)]
pub async fn index() -> std::io::Result<NamedFile> {
    NamedFile::open("static/index.html").await
}
