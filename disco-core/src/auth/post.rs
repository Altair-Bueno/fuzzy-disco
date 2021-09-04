use mongodb::Collection;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::State;

use crate::api::result::ApiResult;
use crate::auth::new_user::NewUser;
use crate::mongo::traits::IntoDocument;
use crate::mongo::user::User;


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

#[post("/signup",format = "json",data = "<user>")]
pub async fn signup(user: Json<NewUser<'_>>, mongo: &State<Collection<User>>) -> ApiResult {
    let user = match user.0.validate() {
        Ok(x) => x,
        Err(x) => return Custom(Status::BadRequest,json!({"status":"Bad Request","message":x}))
    };
    let mongo_response = mongo.insert_one(user,None).await;
    match mongo_response {
        Ok(_) => {
            Custom(Status::Created , json!({"status":"Created","message": "User created"}))
        }
        Err(_) =>  {
            // fixme check if it is colision or db connection error
            Custom(Status::Conflict,json!({"status":"Conflict","message": "Alias taken"}))
        }
    }
}