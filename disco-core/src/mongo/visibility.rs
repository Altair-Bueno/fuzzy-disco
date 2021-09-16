use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Marks posts and media as `Public` or `Private`
#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum Visibility {
    Private,
    Public,
}

impl FromStr for Visibility {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Private" => Ok(Visibility::Private),
            "Public" => Ok(Visibility::Public),
            _ => Err(()),
        }
    }
}

impl From<Visibility> for mongodb::bson::Bson {
    fn from(v: Visibility) -> Self {
        mongodb::bson::to_bson(&v).unwrap()
    }
}


impl Default for Visibility {
    fn default() -> Self {
        Visibility::Private
    }
}
