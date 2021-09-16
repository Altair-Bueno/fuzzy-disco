use serde::Deserialize;
use serde::Serialize;
use crate::api::data::ApiDate;

#[derive(Debug,Serialize,Deserialize, FromForm)]
pub struct Window {
    #[field(default = 0)]
    pub get: u8,
    #[field(default = 0)]
    pub drop:usize,
}
