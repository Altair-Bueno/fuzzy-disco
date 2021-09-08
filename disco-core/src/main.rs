#[macro_use]
extern crate rocket;

use std::option::Option::Some;
use std::sync::Arc;

use dashmap::DashMap;
use rocket::fairing::AdHoc;
use rocket::fs::FileServer;

mod api;
mod init;
mod mongo;

pub type CacheFiles = Arc<DashMap<String, String>>;

#[rocket::main]
async fn main() -> Result<(), String> {
    // Setting up mongodb connection
    println!("Connecting to database...");
    let mongo_database = match init::init_mongo_db().await {
        Ok(client) => client,
        Err(err) => return Err(format!("{:?}", err)),
    };
    println!("Database connection sucessfull");
    println!("Starting up disco-core...");

    let mongo_user_collection = mongo_database.collection::<mongo::user::User>("Users");
    let mongo_post_collection = mongo_database.collection::<mongo::post::Post>("Posts");
    let mongo_media_collection = mongo_database.collection::<mongo::media::Media>("Media");
    let mongo_session_collection = mongo_database.collection::<mongo::session::session>("sessions");

    // Setting up Redis connection
    // todo https://docs.rs/redis/0.21.1/redis/

    // Create Hashmap for temporal files
    // TODO use redis instead
    if let Err(x) = rocket::tokio::fs::create_dir("temp/").await {
        #[cfg(debug_assertions)]
        println!("{}", x)
    }

    let temporal_files: CacheFiles = Arc::new(dashmap::DashMap::new());
    let (sender, mut reciver) = rocket::tokio::sync::mpsc::channel::<String>(100);
    let _ = rocket::tokio::spawn(async move {
        #[cfg(debug_assertions)]
        println!("[GC]: Waiting for expired files");
        while let Some(expired) = reciver.recv().await {
            #[cfg(debug_assertions)]
            println!("[GC]: Removing {}", expired);
            let _ = rocket::tokio::fs::remove_file(expired).await;
        }
        #[cfg(debug_assertions)]
        println!("[GC]: Cleanup");
        let _ = rocket::tokio::fs::remove_dir_all("temp/").await;
    });

    // launch Rocket server
    let rocket_result = rocket::build()
        // Shared state and db connections
        .manage(mongo_user_collection)
        .manage(mongo_post_collection)
        .manage(mongo_media_collection)
        .manage(mongo_session_collection)
        .manage(temporal_files)
        .manage(sender)
        // Mounted routes
        .mount(
            "/api/posts",
            routes![
                api::posts::get::get_post_content,
                api::posts::get::get_posts,
            ],
        )
        .mount("/api/media", routes![api::media::post::upload,])
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
                api::users::put::update_user_password,
                api::users::put::update_user_info,
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
        .mount("/api/media", FileServer::from("media")) // TODO Auth media
        .mount("/", FileServer::from("static").rank(11))
        //.attach(AdHoc::on_request("Response",|x,_| Box::pin(async move { println!("Request: {:#?}",x)})))
        .launch()
        .await;
    match rocket_result {
        Err(e) => Err(format!("{:?}", e)),
        _ => Ok(()),
    }
}
