use crate::api::result::ApiResult;
use crate::mongo::user::{Alias, User};
use mongodb::bson::doc;
use mongodb::Collection;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::State;
use rocket::serde::json::Value;
use crate::api::users::auth::token::claims::TokenClaims;

/// # `GET /api/users/<alias>`
/// Returns the public information avaliable for the given user
///
/// # Returns
/// ## Ok (200)
///
/// ```json
/// {
///     "alias": String,
///     "posts": [String],
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
/// | 400 | `alias` isn't correctly formated |
/// | 404 | User doesn't exist |
/// | 500 | Couldn't connect to database |
///
/// # Example
///
/// `GET /api/users/altair-bueno`
///
/// ```json
/// {
///  "alias": "altair-bueno",
///  "posts": [
///     "6132137e6c2cc66344ef2a88"
///  ]
///}
/// ```
#[get("/<alias>")]
pub async fn get_user_info(alias: &str, mongo: &State<Collection<User>>) -> ApiResult {
    let alias = match alias.parse::<Alias>() {
        Ok(x) => x,
        Err(_) => {
            return Custom(
                Status::BadRequest,
                json!({"status":Status::BadRequest.reason(),"message": "Invalid alias"}),
            )
        }
    };
    let user = match locate_user(&alias, mongo).await {
        Ok(a) => a,
        Err(b) => return b
    };
    Custom(
        Status::Ok,
        json!({
            "alias": user.alias(),
            "posts": user.posts(),
        }
    ))
}
/// # AUTH! `GET /api/users`
/// Returns the **private** information stored about the user. This includes
/// everything except the hashed password
///
/// # Returns
/// ## Ok (200)
///
/// ```json
/// {
///     "alias": String,
///     "posts": [String],
///     "email": String,
///     "creation_date": Date
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
/// | 404 | User doesn't exist |
/// | 500 | Couldn't connect to database |
///
/// # Example
///
/// `GET /api/users/altair-bueno`
///
/// ```json
/// {
///   "alias": "helloworld",
///   "posts": [
///   ],
///   "email": "e@hello.es",
///   "creation_date": "2021-09-06 16:13:02.797 UTC"
/// }
/// ```

#[get("/")]
pub async fn get_full_user_info(mongo: &State<Collection<User>>,token: TokenClaims) -> ApiResult {
    let user = match locate_user(token.alias(), mongo).await {
        Ok(user) => user,
        Err(err) => return err
    };
    Custom(
        Status::Ok,
        json!({
            "alias": user.alias(),
            "posts": user.posts(),
            "email": user.email(),
            "creation_date": user.creation_date().to_string()
        })
    )
}

async fn locate_user(alias: &Alias, mongo: &State<Collection<User>>) -> Result<User,Custom<Value>> {
    let result = mongo
        .find_one(doc! {"alias": alias.to_string() }, None)
        .await;
    match result {
        Ok(Some(x)) => Ok(x),
        Ok(None) => Err(Custom(
            Status::NotFound,
            json!({"status":Status::NotFound.reason(),"message": "User doesn't exist"}),
        )),
        Err(_) => Err(Custom(
            Status::InternalServerError,
            json!({"status": Status::InternalServerError.reason(),"message": "Couldn't connect to database"}),
        )),
    }
}