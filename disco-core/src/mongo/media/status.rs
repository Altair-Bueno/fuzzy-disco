use serde::{Serialize,Deserialize};
use mongodb::bson::DateTime;

#[derive(Serialize,Deserialize,PartialOrd, PartialEq,Ord, Eq,Debug)]
pub enum Status {
    TTL { ttl: DateTime },
    Accepted,
    TimedOut,
}