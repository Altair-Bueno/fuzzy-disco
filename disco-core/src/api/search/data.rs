use serde::Deserialize;
use serde::Serialize;

#[derive(Debug,Serialize,Deserialize, FromForm)]
pub struct Window {
    #[field(default = 0)]
    pub get: u8,
    #[field(default = 0)]
    pub drop:usize,
}
