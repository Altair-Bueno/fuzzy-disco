use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use crate::mongo::user::Session;

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicsessionData {
    ip: Option<IpAddr>,
    date: String,
}

impl PublicsessionData {
    pub fn new(ip: Option<IpAddr>, date: String) -> Self {
        PublicsessionData { ip, date }
    }
    pub fn from_session(session: Session) -> Self {
        PublicsessionData {
            ip: session.ip(),
            date: session.date().to_string(),
        }
    }
}
