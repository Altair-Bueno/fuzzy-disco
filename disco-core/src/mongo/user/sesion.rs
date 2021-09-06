use mongodb::bson::DateTime;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, PartialEq, Eq, Clone, Hash)]
pub struct Sesion {
    id: String,
    date: String,
}

impl Sesion {
    pub fn new() -> Sesion {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        Sesion {
            id: s,
            date: DateTime::now().to_string(),
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn date(&self) -> &str {
        &self.date
    }
}
