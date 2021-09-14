use serde::{Deserialize, Serialize};

use crate::mongo::user::Session;

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicSessionData {
    ip: Option<String>,
    date: String,
}

impl PublicSessionData {
    pub fn new(ip: Option<String>, date: String) -> Self {
        PublicSessionData { ip, date }
    }
    pub fn from_session(session: Session) -> Self {
        PublicSessionData {
            ip: session.ip().clone(),
            date: session.date().to_string(),
        }
    }
}
