use mongodb::Collection;
use mongodb::bson::doc;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::result::ApiResult;
use crate::auth::data::{UserSingUp, UserLogInEmail, UserLogInAlias};
use crate::mongo::IntoDocument;
use crate::mongo::user::{User, Email, Password, UserError, Alias};
use crate::auth::Token;

/// # `POST /auth/signup`
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
            )
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

/// # `POST /auth/login?using=<method>`
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
///     "token": "sadfj.,q928ur23512x-u8124cux1zpU89(/>Z(SFD89A23XCQ"
/// }
/// ```
#[post("/login?using=email", format = "json", data = "<info>")]
pub async fn login_email(info:Json<UserLogInEmail<'_>>, mongo: &State<Collection<User>>) -> ApiResult {
    let email = info.email.parse::<Email>();
    let password = info.password.parse::<Password>();
    let search = match (email,password) {
        (Ok(e), Ok(p)) => mongo.find_one(Some(doc! {"email": e.email(), "password": p.password()}),None).await,
        (Err(x),_) => return Custom(Status::BadRequest, json!({"status": "BadRequest", "message": x})),
        (_,Err(x)) => return Custom(Status::BadRequest,json!({"status":"BadRequest","message":x})),
    };
    create_token(search)
}
#[post("/login?using=alias", format = "json", data = "<info>", rank = 2)]
pub async fn login_alias(info:Json<UserLogInAlias<'_>>, mongo: &State<Collection<User>>) -> ApiResult {
    let alias = info.alias.parse::<Alias>();
    let password = info.password.parse::<Password>();
    let search = match (alias,password) {
        (Ok(a), Ok(p)) => mongo.find_one(Some(doc! {"alias": a.alias(), "password": p.password()}),None).await,
        (Err(x),_) => return Custom(Status::BadRequest, json!({"status": "BadRequest", "message": x})),
        (_,Err(x)) => return Custom(Status::BadRequest,json!({"status":"BadRequest","message":x})),
    };
    create_token(search)
}

fn create_token(result: mongodb::error::Result<Option<User>>) -> ApiResult {
    match result {
        Ok(Some(x)) => Custom(Status::Ok, json!({"status":"Ok","token": Token::new_encrypted(x.id().unwrap())})),
        Ok(None) => Custom(Status::Unauthorized, json!({"status":"Unauthorized", "message": "User doesn't exist"})),
        _ => Custom(Status::InternalServerError,json!({"status":"InternalServerError","message": "Database error"}))
    }
}