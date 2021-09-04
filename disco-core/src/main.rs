#[macro_use]
extern crate rocket;


use rocket::fs::FileServer;
use dashmap::DashMap;
use std::future::Future;


mod api;
mod auth;
mod init;
mod mongo;

pub type CacheFiles = DashMap<String,String>;

#[rocket::main]
async fn main() -> Result<(), String> {
    // Setting up mongodb connection
    let mongodb_client = match init::init_mongo_client().await {
        Ok(client) => client,
        Err(err) => return Err(format!("{:?}", err)),
    };
    let mongo_database = mongodb_client.database("fuzzy-disco");
    let mongo_user_collection = mongo_database.collection::<mongo::user::User>("Users");
    let mongo_post_collection = mongo_database.collection::<mongo::post::Post>("Posts");
    let mongo_media_collection = mongo_database.collection::<mongo::media::Media>("Media");

    // Setting up Redis connection
    // todo https://docs.rs/redis/0.21.1/redis/

    // Create Hashmap for temporal files
    if let Err(x) = rocket::tokio::fs::create_dir("temp/").await {
        println!("{}",x)
    }

    let temporal_files: CacheFiles = dashmap::DashMap::new();
    // launch Rocket server
    let rocket_result = rocket::build()
        // Shared state and db connections
        .manage(mongo_user_collection)
        .manage(mongo_post_collection)
        .manage(mongo_media_collection)
        .manage(temporal_files)
        // Mounted routes
        .mount(
            "/api/posts",
            routes![
                api::posts::get::get_post_content,
                api::posts::get::get_posts,
            ],
        )
        //.mount("/api/users/", routes![])
        .mount(
            "/api/media",
            routes![
                api::media::put::upload,
            ]
        )
        .mount("/api/media", FileServer::from("media"))
        .mount("/", FileServer::from("static").rank(11))
        .launch()
        .await;
    match rocket_result {
        Err(e) => Err(format!("{:?}", e)),
        _ => Ok(()),
    }
}
