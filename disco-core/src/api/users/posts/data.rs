use serde::Serialize;
use serde::Deserialize;
use mongodb::bson::oid::ObjectId;
#[derive(Serialize,Deserialize)]
pub struct Payload{
    #[serde(rename = "_id")]
    pub(crate) id: ObjectId
}