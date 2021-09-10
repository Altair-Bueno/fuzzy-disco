use serde::{Deserialize, Serialize};

use crate::mongo::user::Session;

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicsessionData {
    ip: Option<String>,
    date: String,
}

impl PublicsessionData {
    pub fn new(ip: Option<String>, date: String) -> Self {
        PublicsessionData { ip, date }
    }
    pub fn from_session(session: Session) -> Self {
        PublicsessionData {
            ip: session.ip().clone(),
            date: session.date().to_string(),
        }
    }
}
