use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, PartialEq, Eq, Clone, Hash)]
pub struct Sesion {
    // sesion token
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    // subject object id
    sub: ObjectId,
    // date
    date: DateTime,
}

impl Sesion {
    pub fn new(sub: ObjectId) -> Sesion {
        Sesion {
            id: None,
            sub,
            date: DateTime::now(),
        }
    }

    pub fn id(&self) -> Option<ObjectId> {
        self.id
    }
    pub fn sub(&self) -> ObjectId {
        self.sub
    }
    pub fn date(&self) -> DateTime {
        self.date
    }
}
