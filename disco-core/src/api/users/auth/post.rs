use mongodb::bson::doc;
use mongodb::Collection;
use rocket::http::{ContentType, Header, Status};
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket::{Response, State};

use crate::api::result::ApiResult;
use crate::api::users::auth::data::{
    JoinedRefreshToken, UserLogInAlias, UserLogInEmail, UserLogInRefreshToken, UserSingUp,
};
use crate::api::users::auth::token::claims::TokenClaims;
use crate::api::users::auth::token::response::TokenResponse;
use crate::mongo::sesion::Sesion;
use crate::mongo::user::{Alias, Email, User};
use crate::mongo::IntoDocument;
use rocket::serde::json::Value;
use std::io::Cursor;
use std::str::FromStr;

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
) -> Result<TokenResponse, Custom<Value>> {
    let user_result = match info.email.parse::<Email>() {
        Ok(e) => {
            user_collection
                .find_one(Some(doc! {"email": e.email()}), None)
                .await
        }
        Err(x) => {
            return Err(Custom(
                Status::BadRequest,
                json!({"status": "BadRequest", "message": x}),
            ))
        }
    };

    let x = check_user(user_result).await?;
    verify_password(&x, info.password).await?;
    create_sesion(x,session_collection).await
}

#[post("/login?using=alias", format = "json", data = "<info>", rank = 2)]
pub async fn login_alias(
    info: Json<UserLogInAlias<'_>>,
    user_collection: &State<Collection<User>>,
    session_collection: &State<Collection<Sesion>>,
) -> Result<TokenResponse, Custom<Value>> {
    let user_result = match info.alias.parse::<Alias>() {
        Ok(e) => {
            user_collection
                .find_one(Some(doc! {"alias": e.alias()}), None)
                .await
        }
        Err(x) => {
            return Err(Custom(
                Status::BadRequest,
                json!({"status": "BadRequest", "message": x}),
            ))
        }
    };

    let x = check_user(user_result).await?;
    verify_password(&x, info.password).await?;
    create_sesion(x,session_collection).await
}

async fn create_sesion(
    user: User,
    session_collection: &State<Collection<Sesion>>,
) -> Result<TokenResponse, Custom<Value>> {
    let sesion = Sesion::new(user.alias().clone());
    match session_collection.insert_one(&sesion, None).await {
        Ok(x) => {
            let sesion: mongodb::bson::oid::ObjectId =
                mongodb::bson::from_bson(x.inserted_id).unwrap();
            let (expires, payload) = match TokenClaims::new_encrypted(user.alias().clone())
            {
                Ok(elem) => elem,
                Err(_) => {
                    return Err(Custom(
                        Status::InternalServerError,
                        json!({
                                    "status": Status::InternalServerError.reason(),
                                    "message": "Couldn't generate user token"
                                }),
                    ))
                }
            };
            Ok(TokenResponse::new(expires, sesion.to_string(), payload))
        }
        _ => Err(Custom(
            Status::InternalServerError,
            json!({"status": Status::InternalServerError.reason(),
                "message": "Couldn't generate session token"}),
        )),
    }
}
async fn verify_password(
    user: &User,
    password: &str,
) -> Result<(), Custom<Value>> {
    match user.password().validate(password) {
        Ok(true) => Ok(()),
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
) -> Result<User, Custom<Value>> {
    match result {
        Ok(Some(x)) => Ok(x),
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
/*
/////////////////////////////////
/////////////////////////////////
///////////// DANGER
/////////////////////////////////
/////////////////////////////////
#[post("/login?using=refresh_token", format = "json", data = "<info>")]
pub async fn login_refresh_token(
    info: Json<UserLogInRefreshToken<'_>>,
    mongo: &State<Collection<Sesion>>,
) -> Result<TokenResponse, Custom<Value>> {
    let oid = match mongodb::bson::oid::ObjectId::from_str(info.refresh_token) {
        Ok(x) => x,
        Err(_) => return Err(Custom(Status::BadRequest,json!({
            "status":Status::BadRequest.reason(),
            "message": "Invalid refresh token"
        })))
    };
    run_session_querry(oid,mongo).await
}

async fn run_session_querry(
    oid: mongodb::bson::oid::ObjectId,
    mongo: &State<Collection<Sesion>>
) -> Result<TokenResponse, Custom<Value>> {
    let pipelineop = doc! {
        "$filter": {
            "input": "$lookup":{
                "from": "Users",
                "localField": "user_alias",
                "foreignField": "alias",
                "as": "users"
            },
            "as": "joined",
            "cond": {
                "$eq": [oid, "$$joined.users._id"]
            }
        }
    };
    let database_error = Err(Custom(Status::InternalServerError,json!({
            "status":Status::InternalServerError.reason(),
            "message": "Database error"
        })));
    let mut joined_result = match mongo.aggregate(pipelineop, None).await {
        Ok(x) => x,
        // TODO
        Err(_) =>return database_error
    };
    match joined_result.next().await {
        Some(Ok(x)) => parse_token(x).await,
        Some(_) => database_error,
        None => Err(Custom(Status::Unauthorized, json!({
            "status": Status::Unauthorized.reason(),
            "message": "Invalid refresh token"
        })))
    }
}
async fn parse_token(x: mongodb::bson::Document) -> Result<TokenResponse, Custom<Value>>{
    // Join is safe
    let joined: JoinedRefreshToken = mongodb::bson::from_document(x).unwrap();
    if let Some(user) = joined.users.first() {
        let (expires,token) = TokenClaims::new_encrypted(user.alias().clone()).unwrap();
        Ok(TokenResponse::new(expires, info.refresh_token.to_string(), token))
    } else {
        let _ = mongo.find_one_and_delete(doc! {"_id": info.refresh_token},None).await;
        Err(Custom(
            Status::BadRequest,
            json!({"status": "BadRequest", "message": x}),
        ))
    }
}
*/