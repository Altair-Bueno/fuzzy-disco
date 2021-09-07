#[macro_use]
extern crate rocket;

use std::option::Option::Some;
use std::sync::Arc;

use dashmap::DashMap;
use mongodb::bson::doc;
use rocket::fairing::AdHoc;
use rocket::fs::FileServer;

mod api;
mod init;
mod mongo;

pub type CacheFiles = Arc<DashMap<String, String>>;

#[rocket::main]
async fn main() -> Result<(), String> {
    // Setting up mongodb connection
    let mongodb_client = match init::init_mongo_client().await {
        Ok(client) => client,
        Err(err) => return Err(format!("{:?}", err)),
    };
    let mongo_database = mongodb_client.database("fuzzy-disco");
    // FIXME rust driver version 2.0 should allow index creation more easily
    let index_response = mongo_database
        .run_command(
            doc! {
                "createIndexes": "Users",
                "indexes": [
                    {
                        "key": { "alias": 1 },
                        "name": "alias",
                        "unique": true
                    },
                    {
                        "key": { "email": 1 },
                        "name": "email",
                        "unique": false
                    }
                ]
            },
            None,
        )
        .await;
    #[cfg(debug_assertions)]
    println!("[MONGO] {:?}", index_response);
    let index_response = mongo_database
        .run_command(
            doc! {
                "createIndexes": "Sesions",
                "indexes": [
                    {
                        "key": { "sub": 1 },
                        "name": "sub",
                        "unique": false
                    },
                ]
            },
            None,
        )
        .await;

    #[cfg(debug_assertions)]
    println!("[MONGO] {:?}", index_response);

    let mongo_user_collection = mongo_database.collection::<mongo::user::User>("Users");
    let mongo_post_collection = mongo_database.collection::<mongo::post::Post>("Posts");
    let mongo_media_collection = mongo_database.collection::<mongo::media::Media>("Media");
    let mongo_sesion_collection = mongo_database.collection::<mongo::sesion::Sesion>("Sesions");

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
        .manage(mongo_sesion_collection)
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
                api::users::auth::post::login_email
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
