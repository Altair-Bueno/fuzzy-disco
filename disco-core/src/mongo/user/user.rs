use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::mongo::user::alias::Alias;
use crate::mongo::user::password::Password;

/// Represents a stored document on a document based database such as MongoDB.
/// Althought JSON does not enforce any kind of schema, Rust type safety allows
/// us to enforce certain rules
///
/// # Valid document
///
/// A User document is considered to be **valid** when all of his childs are
/// valid too. For more information check their childs
/// - [mongodb::bson::oid::ObjectId]
/// - [crate::mongo::user::alias::Alias]
/// - [crate::mongo::user::password::Password]
/// - [mongodb::bson::DateTime]
#[derive(Debug, Serialize, Deserialize, Validate, Ord, PartialOrd, PartialEq, Eq)]
pub struct User {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,

    #[serde(flatten)]
    #[validate]
    alias: Alias,

    #[serde(flatten)]
    #[validate]
    password: Password,

    posts_id: Vec<ObjectId>,
    creation_date: DateTime,
}

impl User {
    /// Creates a new user with the current time and empty list of posts
    pub fn new(alias: Alias, password: Password) -> Self {
        User {
            id: None,
            alias,
            password,
            posts_id: vec![],
            creation_date: mongodb::bson::DateTime::now(),
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
    pub fn posts_id(&self) -> &Vec<ObjectId> {
        &self.posts_id
    }
    pub fn creation_date(&self) -> DateTime {
        self.creation_date
    }
}

#[cfg(test)]
mod test {
    use crate::mongo::user::password::Password;
    use crate::mongo::user::user::User;

    #[test]
    pub fn deserialization() {
        /*
        Original YAML
        _id: "612e1913c63244f6b37b1c8f"
        alias: "Altair-Bueno"
        password: "123456789dasfwa"
        posts_id:
        - "632e1913c63244f6b37b1c8f"
        creation_date: 2021-09-01 22:29:13.769 UTC
        */
        let json = "{
  \"_id\": \"612e1913c63244f6b37b1c8f\",
  \"alias\": \"Altair-Bueno\",
  \"password\": \"123456789dasfwa\",
  \"posts_id\": [
    \"632e1913c63244f6b37b1c8f\"
  ],
  \"creation_date\": {\"$date\":{\"$numberLong\":\"1630535430467\"}}
}";
        let _: User = serde_json::from_str(json).unwrap();
    }

    #[test]
    pub fn serlialization() {
        let user = User::new(
            "Altair-Bueno".parse().unwrap(),
            Password::new("Helloworld").unwrap(),
        );
        let ser = serde_json::to_string(&user).unwrap();
        let des: User = serde_json::from_str(ser.as_str()).unwrap();

        assert_eq!(des, user)
    }
}
