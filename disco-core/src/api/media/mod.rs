/// POST /api/media
pub mod post;
pub mod data;

use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use crate::mongo::media::{Media, Format};
use mongodb::bson::doc;
use crate::api::result::ApiError;

pub async fn claim_media(oid: ObjectId,collection: Collection<Media>,expected:Format) -> mongodb::error::Result<Option<Media>> {
    let filter = doc! { "_id": oid , "status": "Waiting", "format": mongodb::bson::to_document(&expected).unwrap() };
    let update = doc! { "$set": { "status": "Assigned" } };
    collection.find_one_and_update(filter,update,None).await
}
