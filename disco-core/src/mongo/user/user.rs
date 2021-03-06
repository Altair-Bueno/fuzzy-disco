use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

use crate::mongo::traits::Document;
use crate::mongo::user::alias::Alias;
use crate::mongo::user::email::Email;
use crate::mongo::user::password::Password;
use crate::mongo::user::Description;

/// Represents a stored document on a document based database such as MongoDB.
/// Althought JSON does not enforce any kind of schema, Rust type safety allows
/// us to enforce certain rules
///
/// # Valid document
///
/// A User document is considered to be **valid** when all of his childs are
/// valid too. For more information check their childs
/// - [mongodb::bson::oid::ObjectId]
/// - [crate::mongo::user::Alias]
/// - [crate::mongo::user::Password]
/// - [mongodb::bson::DateTime]
/// - [crate::mongo::post::Caption]
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    alias: Alias,
    email: Email,
    password: Password,
    description: Option<Description>,
    creation_date: DateTime,
    avatar: Option<ObjectId>,
}

impl Document for User {}

impl User {
    /// Creates a new user with the current time and empty list of posts
    pub fn new(alias: Alias, email: Email, password: Password) -> Self {
        User {
            id: None,
            alias,
            email,
            password,
            description: None,
            creation_date: mongodb::bson::DateTime::now(),
            avatar: None,
        }
    }

    pub fn id(&self) -> Option<ObjectId> {
        self.id
    }
    pub fn alias(&self) -> &Alias {
        &self.alias
    }
    pub fn password(&self) -> &Password {
        &self.password
    }
    pub fn creation_date(&self) -> DateTime {
        self.creation_date
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn description(&self) -> &Option<Description> {
        &self.description
    }
    pub fn avatar(&self) -> Option<ObjectId> {
        self.avatar
    }
}

#[cfg(test)]
mod test {
    use crate::mongo::user::user::User;

    #[test]
    pub fn deserialization() {
        let json = "{\"alias\":\"Altair-Bueno\",\"email\":\"hello@world.com\",\"password\":\"$2b$12$NpqbpxgCy2EN6sdm/3YB4eRGfn1LdPbeMPHoxHW3bpQqAiytYDn46\",\"creation_date\":{\"$date\":{\"$numberLong\":\"1630711570146\"}}}";
        let _: User = serde_json::from_str(json).unwrap();
    }

    #[test]
    pub fn serlialization() {
        let user = User::new(
            "Altair-Bueno".parse().unwrap(),
            "hello@world.com".parse().unwrap(),
            "helloworld".parse().unwrap(),
        );
        let ser = serde_json::to_string(&user).unwrap();
        let des: User = serde_json::from_str(ser.as_str()).unwrap();
        assert_eq!(des, user)
    }
}
