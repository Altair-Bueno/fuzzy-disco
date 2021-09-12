use rocket::FromForm;
use chrono::{Date, Utc};

#[derive(FromForm)]
pub struct SearchQuery <'a> {
    s: &'a str,
    #[field(default = 0)]
    drop: u64,
    get: u64,
    date: &'a str,
}