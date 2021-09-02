#[macro_use]
extern crate rocket;

use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use rocket::futures::TryStreamExt;

use crate::mongo::post::Post;
use crate::mongo::post::Title;
use rocket::fs::FileServer;

mod mongo;
mod api;
mod auth;

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/",FileServer::from("static"))
        .launch()
        .await;

    /*
    let mongo_username_pass = std::env::var("MONGO_USER_PASS")
        .map(|x| format!("{}@", x))
        .unwrap_or("".to_string());
    let mongo_at = std::env::var("MONGO_IP_PORT").unwrap_or("0.0.0.0:27019".to_string());

    let url = format!("mongodb+srv://{}{}/", mongo_username_pass, mongo_at);

    let mongo = mongodb::Client::with_uri_str(url).await.unwrap();
    let collection = mongo.database("Test").collection::<Post>("Test");
    let doc = Post::new(
        "Hello world".parse().unwrap(),
        "Caption text".parse().unwrap(),
        "joselito el panadero".parse().unwrap(),
        "/path/to/file".to_string(),
        "path/to/photo".to_string(),
    );
    let res = collection.insert_one(doc, None).await;
    println!("{:?}", res);

    let filter = doc! { "title": "Hello world" };
    let mut cursor = collection.find(filter, None).await.unwrap();

    while let Some(p) = cursor.try_next().await.unwrap() {
        println!("title: {:?}", p);
    }

     */
}
