use mongodb::bson::doc;
use mongodb::Collection;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::result::ApiResult;
use crate::api::users::auth::data::{UserLogInAlias, UserLogInEmail, UserSingUp, Claims};
use crate::mongo::user::{Alias, Email, User};
use crate::mongo::IntoDocument;

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
pub async fn login_email(
    info: Json<UserLogInEmail<'_>>,
    mongo: &State<Collection<User>>,
) -> ApiResult {
    let search = match info.email.parse::<Email>() {
        Ok(e) => mongo.find_one(Some(doc! {"email": e.email()}), None).await,
        Err(x) => {
            return Custom(
                Status::BadRequest,
                json!({"status": "BadRequest", "message": x}),
            )
        }
    };

    create_token(search, info.password)
}

#[post("/login?using=alias", format = "json", data = "<info>", rank = 2)]
pub async fn login_alias(
    info: Json<UserLogInAlias<'_>>,
    mongo: &State<Collection<User>>,
) -> ApiResult {
    let search = match info.alias.parse::<Alias>() {
        Ok(e) => mongo.find_one(Some(doc! {"alias": e.alias()}), None).await,
        Err(x) => {
            return Custom(
                Status::BadRequest,
                json!({"status": "BadRequest", "message": x}),
            )
        }
    };

    create_token(search, info.password)
}

fn create_token(result: mongodb::error::Result<Option<User>>, password: &str) -> ApiResult {
    match result {
        Ok(Some(x)) => match bcrypt::verify(password, x.password().password()) {
            Ok(true) => match Claims::new_encrypted((*x.alias()).clone()) {
                Ok((expires,payload)) => Custom(Status::Ok, json!({"status":"Ok","expires": expires, "token": payload})),
                Err(_) => Custom(
                    Status::InternalServerError,
                    json!({"status": "InternalServerError", "message": "Couldn't generate token"}),
                ),
            },
            Ok(false) => Custom(
                Status::Unauthorized,
                json!({"status": "Unauthorized", "message": "Invalid password"}),
            ),
            Err(_) => Custom(
                Status::InternalServerError,
                json!({"status":"InternalServerError", "message": "Couldn't verfiy password"}),
            ),
        },
        Ok(None) => Custom(
            Status::Unauthorized,
            json!({"status":"Unauthorized", "message": "User doesn't exist"}),
        ),
        _ => Custom(
            Status::InternalServerError,
            json!({"status":"InternalServerError","message": "Database error"}),
        ),
    }
}
