use mongodb::bson::doc;
use mongodb::Collection;
use rocket::http::{ContentType, Header, Status};
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket::{Response, State};

use crate::api::result::ApiResult;
use crate::api::users::auth::data::{Claims, UserLogInAlias, UserLogInEmail, UserSingUp, Token};
use crate::mongo::user::{Alias, Email, User};
use crate::mongo::IntoDocument;
use bcrypt::BcryptResult;
use rocket::serde::json::Value;
use std::io::Cursor;

/// # `POST api/users/auth/signup`
/// Creates a new user with the recived information. The body for the request
/// must be **JSON** formated with the following content:
///
/// ```json
/// {
///     "alias": String,
///     "email": String,
///     "password": String,
/// }
/// ```
///
/// Each field must follow the user requirements descrived on [User](crate::mongo::user::User)
///
///
/// # Returns
/// ## Ok (201)
///
/// ```json
/// {
///     "status": "Created",
///     "message": "User created"
/// }
/// ```
///
/// ## Err
/// ```json
/// {
///     "status": String,
///     "message": String
/// }
/// ```
///
/// | Code | Description |
/// | -----| ----------- |
/// | 400 | `id` isn't correctly formated |
/// | 409 | Another user already has the same alias |
/// | 500 | Database error |
///
/// # Example
///
/// `POST /auth/signup`
///
/// ## Body payload
///
/// ```json
/// {
///     "alias": "Altair-Bueno",
///     "email": "hello@world.org",
///     "password": "i-love-rvst"
/// }
/// ```
///
/// ## Response
///
/// ```json
/// {
///     "status": "Created",
///     "message": "User created"
/// }
/// ```
#[post("/signup", format = "json", data = "<user>")]
pub async fn signup(user: Json<UserSingUp<'_>>, mongo: &State<Collection<User>>) -> ApiResult {
    let user = match user.0.validate() {
        Ok(x) => x,
        Err(x) => {
            return Custom(
                Status::BadRequest,
                json!({"status":"BadRequest","message":x}),
            );
        }
    };
    let mongo_response = mongo.insert_one(user, None).await;
    match mongo_response {
        Ok(_) => Custom(
            Status::Created,
            json!({"status":"Created","message": "User created"}),
        ),
        Err(_) => {
            // fixme check if it is colision or db connection error
            Custom(
                Status::Conflict,
                json!({"status":"Conflict","message": "Alias taken"}),
            )
        }
    }
}

/// # `POST api/users/auth/login?using=<method>`
/// Returns a JWT for user authentication. The token must be included on the
/// `Authorization` HTTP header for authenticated requests. You can authenticate
/// by either the user alias (method `alias`) or by user email (method `email`).
///
/// ## Alias
/// ```json
/// {
///     "alias": String,
///     "password": String,
/// }
/// ```
///
/// ## Email
/// ```json
/// {
///     "email": String,
///     "password": String,
/// }
/// ```
///
/// # Returns
/// ## Ok (200)
///
/// ```json
/// {
///     "status": "Ok",
///     "token": String,
///     "expires": Date,
/// }
/// ```
///
/// ## Err
/// ```json
/// {
///     "status": String,
///     "message": String
/// }
/// ```
///
/// | Code | Description |
/// | -----| ----------- |
/// | 400 | Alias/email or password incorrect |
/// | 409 | Another user already has the same alias |
/// | 500 | Database error |
///
/// # Example
///
/// `POST /auth/login?using=alias`
///
/// ## Body payload
///
/// ```json
/// {
///     "alias": "Altair-Bueno",
///     "password": "i-love-rvst"
/// }
/// ```
///
/// ## Response
///
/// ```json
/// {
///     "status": "Ok",
///     "expires": "2021-09-05T13:27:50.936160Z",
///     "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjp7IiRvaWQiOiI2MTM0YmZlMTU5MTZmNTJiMTc5OGRhZjIifSwiY3JlYXRlZCI6IjIwMjEtMDktMDVUMTM6MjI6NTAuOTM2MTYwWiIsImV4cGlyZXMiOiIyMDIxLTA5LTA1VDEzOjI3OjUwLjkzNjE2MFoifQ.15pv2ED-NxStcpFDfqHIgizRqWBoN0g0jtFb89Jjw5c"
/// }
/// ```
#[post("/login?using=email", format = "json", data = "<info>")]
pub async fn login_email<'a>(
    info: Json<UserLogInEmail<'a>>,
    mongo: &State<Collection<User>>,
) -> Result<Token, Custom<Value>> {
    let search = match info.email.parse::<Email>() {
        Ok(e) => mongo.find_one(Some(doc! {"email": e.email()}), None).await,
        Err(x) => {
            return Err(Custom(
                Status::BadRequest,
                json!({"status": "BadRequest", "message": x}),
            ))
        }
    };

    check_user(search, info.password, mongo).await
}

#[post("/login?using=alias", format = "json", data = "<info>", rank = 2)]
pub async fn login_alias(
    info: Json<UserLogInAlias<'_>>,
    mongo: &State<Collection<User>>,
) -> Result<Token, Custom<Value>> {
    let search = match info.alias.parse::<Alias>() {
        Ok(e) => mongo.find_one(Some(doc! {"alias": e.alias()}), None).await,
        Err(x) => {
            return Err(Custom(
                Status::BadRequest,
                json!({"status": "BadRequest", "message": x}),
            ))
        }
    };

    check_user(search, info.password, mongo).await
}

async fn verify_password(
    mut user: User,
    password: &str,
    mongo: &State<Collection<User>>,
) -> Result<Token, Custom<Value>> {
    match bcrypt::verify(password, user.password().password()) {
        Ok(true) => {
            let session = user.add_session();
            let query = doc! {"_id": user.id()};
            let update = doc! {"$push": { "sessions" : mongodb::bson::to_document(&session).unwrap() }};
            match mongo.update_one(query, update, None).await {
                Ok(x) if x.modified_count == 1 => {
                    let (expires,payload) = Claims::new_encrypted(user.alias().clone()).unwrap();
                    Ok(Token::new(expires, session.id().to_string(), payload))
                }
                _ => Err(Custom(
                    Status::InternalServerError,
                    json!({"status": Status::InternalServerError.reason(),
                "message": "Couldn't generate session token"}),
                )),
            }
        }
        Ok(false) => Err(Custom(
            Status::Unauthorized,
            json!({"status": "Unauthorized", "message": "Invalid password"}),
        )),
        Err(_) => Err(Custom(
            Status::InternalServerError,
            json!({"status":"InternalServerError", "message": "Couldn't verfiy password"}),
        )),
    }
}

async fn check_user(
    result: mongodb::error::Result<Option<User>>,
    password: &str,
    mongo: &State<Collection<User>>,
) -> Result<Token, Custom<Value>> {
    match result {
        Ok(Some(x)) => verify_password(x, password, mongo).await,
        Ok(None) => Err(Custom(
            Status::Unauthorized,
            json!({"status":"Unauthorized", "message": "User doesn't exist"}),
        )),
        _ => Err(Custom(
            Status::InternalServerError,
            json!({"status":"InternalServerError","message": "Database error"}),
        )),
    }

}
