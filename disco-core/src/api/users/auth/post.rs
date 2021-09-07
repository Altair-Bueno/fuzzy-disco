use mongodb::bson::doc;
use mongodb::Collection;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::result::ApiError;
use crate::api::users::auth::data::{UserLogInAlias, UserLogInEmail, UserSingUp};
use crate::api::users::auth::token::claims::TokenClaims;
use crate::api::users::auth::token::response::TokenResponse;
use crate::mongo::sesion::Sesion;
use crate::mongo::user::{Alias, Email, User};
use crate::mongo::IntoDocument;
use rocket::serde::json::Value;

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
pub async fn signup(
    user: Json<UserSingUp<'_>>,
    mongo: &State<Collection<User>>,
) -> Result<rocket::response::status::Created<Value>, ApiError> {
    let valid_user = user.0.validate()?;
    mongo
        .insert_one(valid_user, None)
        .await
        .map(|_| {
            rocket::response::status::Created::new(format!("/api/user/{}", user.0.alias))
                .body(json!({"status":"Created","message": "User created"}))
        })
        .map_err(|_| ApiError::Conflict("User Alias"))
    // fixme check if it is colision or db connection error
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
///     "access_token": String,
///     "expires_in": i64,
///     "refresh_token": String,
///     "token_type": "Bearer",
///     "scope": "User Login"
/// }
/// ```
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
/// ```json
/// {
/// "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJoZWxsb3dvcmxkIiwiZXhwIjoxNjMwOTQ0ODg0LCJpYXQiOjE2MzA5NDQ4MjR9.Ux2XbdhHPYvnmnkC8hfUPBsQPpZDtrgm2zbBmMYj1Vo",
/// "expires_in": 60,
/// "refresh_token": "61363e38a8285591b0b79cb2",
/// "token_type": "Bearer",
/// "scope": "User login"
/// }
/// ```
#[post("/login?using=email", format = "json", data = "<info>", rank = 3)]
pub async fn login_email(
    info: Json<UserLogInEmail<'_>>,
    user_collection: &State<Collection<User>>,
    session_collection: &State<Collection<Sesion>>,
) -> Result<TokenResponse, ApiError> {
    let email = info.email.parse::<Email>()?;
    let user = user_collection
        .find_one(Some(doc! {"email": email.email()}), None)
        .await?;
    let x = match user {
        Some(x) => x,
        None => return Err(ApiError::NotFound("User")),
    };
    verify_password(&x, info.password).await?;
    create_sesion(x, session_collection).await
}

#[post("/login?using=alias", format = "json", data = "<info>", rank = 2)]
pub async fn login_alias(
    info: Json<UserLogInAlias<'_>>,
    user_collection: &State<Collection<User>>,
    session_collection: &State<Collection<Sesion>>,
) -> Result<TokenResponse, ApiError> {
    let alias = info.alias.parse::<Alias>()?;
    let user = user_collection
        .find_one(Some(doc! {"alias": alias.alias()}), None)
        .await?;
    let x = match user {
        Some(x) => x,
        None => return Err(ApiError::NotFound("User")),
    };
    verify_password(&x, info.password).await?;
    create_sesion(x, session_collection).await
}

async fn create_sesion(
    user: User,
    session_collection: &State<Collection<Sesion>>,
) -> Result<TokenResponse, ApiError> {
    let sesion = Sesion::new(user.alias().clone());
    let x = session_collection.insert_one(&sesion, None).await?;
    let sesion: mongodb::bson::oid::ObjectId = mongodb::bson::from_bson(x.inserted_id).unwrap();
    let (expires, payload) = TokenClaims::new_encrypted(user.alias().clone());
    Ok(TokenResponse::new(expires, sesion.to_string(), payload))
}
async fn verify_password(user: &User, password: &str) -> Result<(), ApiError> {
    match user.password().validate(password) {
        Ok(true) => Ok(()),
        Ok(false) => Err(ApiError::Unauthorized("Invalid password")),
        Err(_) => Err(ApiError::InternalServerError("Couldn't hash password")),
    }
}
// TODO LOGIN USING REFRESH TOKEN
// POST /api/users/auth/login?using=refresh_token
