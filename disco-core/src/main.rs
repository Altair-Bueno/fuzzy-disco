#[macro_use]
extern crate rocket;


use rocket::fs::FileServer;

use crate::mongo::post::Post;
use crate::mongo::post::Title;

mod api;
mod auth;
mod init;
mod mongo;

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

    // Setting up Redis connection
    // todo https://docs.rs/redis/0.21.1/redis/

    // launch Rocket server
    let rocket_result = rocket::build()
        .manage(mongo_user_collection)
        .manage(mongo_post_collection)
        .mount(
            "/api/posts",
            routes![
                api::posts::get::get_post_content,
                api::posts::get::get_posts,
            ],
        )
        //.mount("/api/users/", routes![])
        .mount("/api/media", FileServer::from("media").rank(9))
        .mount("/", FileServer::from("static"))
        .launch()
        .await;
    match rocket_result {
        Err(e) => Err(format!("{:?}", e)),
        _ => Ok(()),
    }
}
