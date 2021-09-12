use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;

use crate::api::result::{ApiError, ApiResult};
use crate::mongo::media::{Format, Status};
use crate::mongo::user::Alias;
use crate::api::{MEDIA_STATUS, MEDIA_UPLOADED_BY, MEDIA_ID, MEDIA_FORMAT};
/// Data Structures used on this module
mod data;
/// GET /api/media
pub mod get;
/// POST /api/media
pub mod post;

const MEDIA_ROOT_FOLDER: &str = "media/";

pub async fn claim_media_filter(
    oid: &ObjectId,
    expected: &Format,
    uploaded_by: &Alias,
) -> mongodb::bson::Document {
    doc! {
        MEDIA_ID: oid ,
        MEDIA_STATUS: mongodb::bson::to_bson(&Status::Waiting).unwrap(),
        MEDIA_FORMAT: mongodb::bson::to_bson(expected).unwrap(),
        MEDIA_UPLOADED_BY : mongodb::bson::to_bson(uploaded_by).unwrap()
    }
}

pub async fn delete_media(oid: &ObjectId) -> ApiResult<()> {
    let path = oid_to_path(oid);
    rocket::tokio::fs::remove_file(path)
        .await
        .map_err(ApiError::FileTransferError)
}

pub async fn claim_media_update() -> mongodb::bson::Document {
    doc! { "$set": { MEDIA_STATUS: mongodb::bson::to_bson(&Status::Assigned).unwrap() } }
}

pub fn oid_to_path(oid: &mongodb::bson::oid::ObjectId) -> String {
    format!("{}/{}.blob", oid_to_folder(oid), oid)
}

fn oid_to_folder(oid: &mongodb::bson::oid::ObjectId) -> String {
    oid.bytes()
        .iter()
        .fold(MEDIA_ROOT_FOLDER.to_string(), |mut acc, n| {
            acc.push('/');
            acc.push_str(n.to_string().as_str());
            acc
        })
}
