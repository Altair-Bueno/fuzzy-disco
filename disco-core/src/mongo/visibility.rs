use serde::{Serialize,Deserialize};
use std::str::FromStr;

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
            _=> Err(())
        }
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Private
    }
}