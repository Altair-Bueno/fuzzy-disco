use chrono::serde::ts_seconds;
use chrono::{DateTime, Duration, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// 2 days of TTL
const TTL_AUTH: i64 = 2;

#[derive(Debug, Serialize, Deserialize, Eq, PartialOrd, PartialEq, Ord)]
pub struct AuthPayload {
    user_id: ObjectId,
    #[serde(with = "ts_seconds")]
    created: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    expires: DateTime<Utc>,
}

impl AuthPayload {
    pub fn new(user_id: ObjectId) -> Self {
        let created = Utc::now();
        let expires = created + Duration::days(TTL_AUTH);
        AuthPayload {
            user_id,
            created,
            expires,
        }
    }
    pub fn is_valid(&self) -> bool {
        let expires = self.expires;
        let now = Utc::now();
        expires > now
    }

    pub fn user_id(&self) -> ObjectId {
        self.user_id
    }
    pub fn created(&self) -> DateTime<Utc> {
        self.created
    }
    pub fn expires(&self) -> DateTime<Utc> {
        self.expires
    }
}
