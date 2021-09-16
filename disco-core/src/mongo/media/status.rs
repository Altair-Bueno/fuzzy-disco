use serde::{Deserialize, Serialize};

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    Waiting,
    Assigned,
}

impl From<Status> for mongodb::bson::Bson{
    fn from(s: Status) -> Self {
        mongodb::bson::to_bson(&s).unwrap()
    }
}