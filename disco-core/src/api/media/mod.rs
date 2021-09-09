/// POST /api/media
pub mod post;
pub mod data;

use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use crate::mongo::media::{Media, Format, Status};
use mongodb::bson::doc;
use crate::api::result::ApiError;
use crate::mongo::user::Alias;

pub async fn claim_media(
    oid: &ObjectId,
    collection: &Collection<Media>,
    expected:&Format,
    uploaded_by: &Alias
) -> mongodb::error::Result<Option<Media>> {
    let filter = doc! {
        "_id": oid ,
        "status": mongodb::bson::to_bson(&Status::Waiting).unwrap(),
        "format": mongodb::bson::to_bson(expected).unwrap(),
        "uploaded_by" : uploaded_by.to_string()
    };
    let update = doc! { "$set": { "status": mongodb::bson::to_bson(&Status::Assigned).unwrap() } };
    collection.find_one_and_update(filter,update,None).await
}
