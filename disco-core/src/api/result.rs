use rocket::response::status::Custom;
use rocket::serde::json::Value;

pub type ApiResult = Result<Value, Custom<Value>>;
