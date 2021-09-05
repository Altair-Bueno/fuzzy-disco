use serde::{Deserialize, Serialize};

use crate::mongo::user::{User, UserError};
use crate::mongo::IntoDocument;


use chrono::{DateTime, Duration, Utc};
use lazy_static::lazy_static;
use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::Request;

use crate::api::users::auth::result::{AuthError, AuthResult};
use crate::mongo::user::Alias;
use jsonwebtoken::{DecodingKey, Validation, Header, EncodingKey};

/// JWT Time To Live
const TTL_AUTH: i64 = 5;


pub type EncryptedToken = String;
pub type ExpireDate = DateTime<Utc>;

/// Represents a JWT's payload. Visit <https://jwt.io> to learn more about JWT
#[derive(Debug, Serialize, Deserialize, Eq, PartialOrd, PartialEq, Ord,Clone)]
pub struct Claims {
    alias: Alias,
    created: DateTime<Utc>,
    expires: DateTime<Utc>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = Value;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(authen_header) = request.headers().get_one("Authorization") {
            let authen_str = authen_header.to_string();
            if authen_str.starts_with("Bearer") {
                let token = authen_str[6..authen_str.len()].trim();
                if let Ok(token_data) =
                    jsonwebtoken::decode::<Claims>(
                        &token,
                        &DecodingKey::from_secret(include_bytes!("../../../../secret.key")),
                        &Validation{validate_exp: false, ..Default::default()}
                    ) {
                    return if token_data.claims.is_valid() {
                        Outcome::Success(token_data.claims)
                    } else {
                        Outcome::Failure((
                            Status::new(440),
                            // TODO 440 catcher
                            json!({"status": "LoginTimeout", "message": "Sesion has expired"}),
                        ))
                    }
                }
            }
        }

        Outcome::Failure((
            Status::BadRequest,
            json!({"status": Status::BadRequest.reason(), "message": "Invalid token"})
        ))
        /*let token: Option<Claims> = request
            .headers()
            .get("Authorization")
            .next()
            .map(|x| {
                let token = JWT::<Claims, biscuit::Empty>::new_encoded(x);
                let token = token.into_decoded(&signing_secret, SignatureAlgorithm::HS256).unwrap();
                (*token.payload().unwrap()).clone().private
            });
        match token {
            Some(x) if x.is_valid() => Outcome::Success(x),
            Some(_)=>Outcome::Failure((
                Status::new(440),
                json!({"status": "LoginTimeout", "message": "Sesion has expired"}),
            )),
            _ => Outcome::Forward(())
        }*/
        /*match token {
            Some(Ok(token)) if token.is_valid()=> Outcome::Success(token),
            Some(Ok(_)) => Outcome::Failure((
                Status::new(440),
                json!({"status": "LoginTimeout", "message": "Sesion has expired"}),
            )),
            Some(Err(_)) => Outcome::Failure((
                Status::BadRequest,
                json!({"status": "BadRequest","message": "Invalid token"}),
            )),
            None => Outcome::Forward(())
        }*/
        /*match token {
            Ok((header,token)) => Outcome::Success(token),
            Some(Ok(_)) => Outcome::Failure((
                Status::new(440),
                json!({"status": "LoginTimeout", "message": "Sesion has expired"}),
            )),
            Some(Err(_)) => Outcome::Failure((
                Status::BadRequest,
                json!({"status": "BadRequest","message": "Invalid token"}),
            )),
            _ => Outcome::Forward(()),
        }*/
    }
}

impl Claims {
    /// Creates a new JWT that is linked to the user ID on the database
    pub fn new_encrypted(alias: Alias) -> AuthResult<(ExpireDate, EncryptedToken)> {
        let created = Utc::now();
        let expires = created + Duration::minutes(TTL_AUTH);

        let claims = Claims {
            alias,
            created,
            expires
        };
        let token = jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(include_bytes!("../../../../secret.key"))).unwrap();

        Ok((expires,token))
    }
    pub fn is_valid(&self) -> bool {
        let expires = self.expires;
        let now = Utc::now();
        expires > now
    }


    pub fn created(&self) -> DateTime<Utc> {
        self.created
    }
    pub fn expires(&self) -> DateTime<Utc> {
        self.expires
    }
    pub fn alias(&self) -> &Alias {
        &self.alias
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserSingUp<'a> {
    alias: &'a str,
    email: &'a str,
    password: &'a str,
}

impl IntoDocument<User> for UserSingUp<'_> {
    type Err = UserError;

    fn validate(self) -> Result<User, Self::Err> {
        let UserSingUp {
            alias,
            email,
            password,
        } = self;
        let alias = alias.parse()?;
        let email = email.parse()?;
        let password = password.parse()?;
        Ok(User::new(alias, email, password))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogInEmail<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogInAlias<'a> {
    pub alias: &'a str,
    pub password: &'a str,
}

#[cfg(test)]
mod tes {
    use crate::api::users::auth::data::Claims;
    use jsonwebtoken::{DecodingKey,Validation};

    #[test]
    pub fn test() {
        let (_,token) = Claims::new_encrypted("Temp".parse().unwrap()).unwrap();
        println!("{:?}",token);
        let decript =  jsonwebtoken::decode::<Claims>(
            &token,
            &DecodingKey::from_secret(include_bytes!("../../../../secret.key")),
            &Validation{validate_exp: false, ..Validation::default()}
        ).unwrap();
        println!("{:?}",decript)

    }
}