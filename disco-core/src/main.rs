//! This crate contains the source code for `disco-core`, the main component on
//! `fuzzy-disco`. The program mainly focuses on two tasks
//!
//! - Providing a fast and reliable JSON API
//! - Serving a website written in [Vue.js](../disco-vue)
//! 
//! # Required services
//! 
//! - [Mongodb](https://www.mongodb.com/es)
//! - [Redis](https://redis.io)
//!
//! # API
//!
//! You can find the whole documentation for the API under the [`api`](crate::api)
//! module
//!
//! # Build and run
//!
//! 1. Install the rust toolchain from the [official website](https://www.rust-lang.org)
//! 2. Start a Mongodb database. You can either use a Docker container
//! (recommended) or install mongo on your local machine
//! 2. Clone this repo and cd to disco-core
//!
//! ```bash
//! git clone https://github.com/Altair-Bueno/fuzzy-disco
//! cd disco-core
//! ```
//!
//! 3. Set up the following environment variables:
//!
//! ```bash
//! export MONGODB_URI="mongodb://<username>:<password>@<ip>:<port>/"
//! export REDIS_URI="redis://<username>:<password>@<ip>:<port>/"
//! ```
//!
//! 4. Copy your static website to `static/`
//! ```bash
//! cp <static> static/
//! ```
//! 
//! > If you don't have any website, just create an empty `static/` folder
//! 
//!
//! 5. Build and run
//!
//! ```bash
//! # Run on release mode
//! cargo run --release
//! # Run on debug mode
//! cargo run
//! ```
#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;

use init::*;

pub mod api;
mod init;
mod mongo;
mod control;

#[rocket::main]
async fn main() -> Result<(), String> {
    // Setting up redis
    let redis_connection = init_redis().await
        .map_err(|x| format!("{:?}",x))?;

    // Setting up mongodb connection
    println!("Connecting to database...");
    let (mongo_database, _mongo_client) = init_mongo_db().await
        .map_err(|err|format!("{:?}", err))?;

    println!("Database connection successfully");
    println!("Starting up disco-core...");

    let mongo_user_collection = mongo_database.collection::<mongo::user::User>("Users");
    let mongo_post_collection = mongo_database.collection::<mongo::post::Post>("Posts");
    let mongo_media_collection = mongo_database.collection::<mongo::media::Media>("Media");
    let mongo_session_collection = mongo_database.collection::<mongo::session::Session>("Sessions");

    if let Err(x) = rocket::tokio::fs::create_dir("temp/").await {
        #[cfg(debug_assertions)]
        println!("{}", x)
    }

    // launch Rocket server
    rocket::build()
        // DB Collections
        .manage(mongo_user_collection)
        .manage(mongo_post_collection)
        .manage(mongo_media_collection)
        .manage(mongo_session_collection)
        .manage(redis_connection)
        //.manage(mongo_client)
        // Mounted routes
        .mount("/api/search", routes![
            api::search::get::search_user,
            api::search::get::search_post,
        ])
        .mount(
            "/api/posts",
            routes![
                api::posts::get::get_post_content,
                api::posts::get::get_post_content_auth,
                api::posts::post::new_post,
                api::posts::delete::delete_post,
                api::posts::patch::edit_post
            ],
        )
        .mount(
            "/api/media",
            routes![
                api::media::post::upload,
                api::media::get::get_media,
                api::media::get::get_media_auth
            ],
        )
        .mount(
            "/api/users/auth",
            routes![
                api::users::auth::post::signup,
                api::users::auth::post::login_alias,
                api::users::auth::post::login_email,
                api::users::auth::post::login_refresh_token,
            ],
        )
        .mount(
            "/api/users",
            routes![
                api::users::get::get_full_user_info,
                api::users::get::get_user_info,
                api::users::posts::get::get_posts_from,
                api::users::posts::get::get_private_posts_from,
                api::users::post::update_user_password,
                api::users::post::update_user_info,
                api::users::post::update_user_avatar,
                api::users::delete::delete_user,
            ],
        )
        .mount(
            "/api/sessions",
            routes![
                api::sessions::get::get_user_sessions,
                api::sessions::post::delete_all_sessions,
            ],
        )
        // Static website server
        .mount("/", FileServer::from("static").rank(11))
        .mount("/", routes![
            control::index,
            control::api_bad_request,
        ])
        //.attach(AdHoc::on_request("Response",|x,_| Box::pin(async move { println!("Request: {:#?}",x)})))
        .launch()
        .await
        .map_err(|e| format!("{:?}", e))
}
