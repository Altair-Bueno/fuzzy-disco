#[macro_use] extern crate rocket;

use rocket::fs::FileServer;

#[get("/")]
fn index()->&'static str {
   "Hello world"
}

#[launch]
fn rocket() -> _ {
   rocket::build()
       .mount("/",FileServer::from("./static"))
}
