use serde::{Deserialize, Serialize};

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    Waiting,
    Assigned,
}
