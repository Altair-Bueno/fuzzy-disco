use crate::mongo::media::Format;
use crate::mongo::user::Alias;

#[derive(Clone)]
pub struct TemporalFileData {
    pub path: String,
    pub format: Format,
    pub user_alias: Alias
}