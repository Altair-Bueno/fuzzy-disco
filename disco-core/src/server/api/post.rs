use rocket::response::content::Json;

#[get("/<id>")]
pub fn post(id:String) -> Json<crate::api::post::Post> {
    todo!()
}