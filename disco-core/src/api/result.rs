use std::collections::HashMap;

pub type JsonResult<T> = Result<
    rocket::serde::json::Json<T>,
    rocket::response::status::Custom<String>
>;

pub type DictionaryResponse = HashMap<&'static str , String>;