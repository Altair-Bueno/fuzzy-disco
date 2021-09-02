use serde::Serialize;
use serde::Deserialize;
use chrono::{DateTime, Utc};
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, PartialEq, Eq,Default)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    posts_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_date: Option<DateTime<Utc>>
}
impl User {
    pub fn build() -> User {
        Default::default()
    }
    pub fn id(&self) -> &Option<String> {
        &self.id
    }
    pub fn alias(&self) -> &Option<String> {
        &self.alias
    }
    pub fn password(&self) -> &Option<String> {
        &self.password
    }
    pub fn posts_id(&self) -> &Option<Vec<String>> {
        &self.posts_id
    }
    pub fn creation_date(&self) -> Option<DateTime<Utc>> {
        self.creation_date
    }
    pub fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }
    pub fn set_alias(&mut self, alias: Option<String>) {
        self.alias = alias;
    }
    pub fn set_password(&mut self, password: Option<String>) {
        self.password = password;
    }
    pub fn set_posts_id(&mut self, posts_id: Option<Vec<String>>) {
        self.posts_id = posts_id;
    }
    pub fn set_creation_date(&mut self, creation_date: Option<DateTime<Utc>>) {
        self.creation_date = creation_date;
    }
}

#[cfg(test)]
mod test {
    use crate::api::user::User;

    #[test]
    pub fn serializing() {
        let json = "{\"creation_date\": \"2012-04-23T18:25:43.511Z\"}";
        let _:User = serde_json::from_str(json).unwrap();
    }
}