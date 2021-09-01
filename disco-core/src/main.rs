mod post;
#[macro_use] extern crate rocket;

use crate::post::title::Title;
use mongodb::options::ClientOptions;
use crate::post::post::Post;

#[rocket::main]
async fn main(){
    let user= "";
    let password = "";
    let url = format!("mongodb+srv://{}:{}@cluster0.aw4fz.mongodb.net/",user,password);
    let mut options = ClientOptions::parse(url).await.unwrap();
    let client = mongodb::Client::with_options(options).unwrap();

    let mongo_db = client.database("Test");
    let collection = mongo_db.collection::<post::post::Post>("Test");
    let res = collection.insert_one(Post::new("Hello".parse().unwrap(),
                                    "world".to_string(),
                                    "test".to_string(),
                                    "audio/path".to_string(),
                                    "photo/path".to_string()), None).await;

    println!("{:?}",res)
}