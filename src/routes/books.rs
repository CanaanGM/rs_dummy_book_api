use rocket::response::status;
use serde_json::{json, Value};

use crate::auth::BasicAuth;
use rocket_sync_db_pools::database;

extern crate diesel;

#[database("sqlite")]
pub struct DbConn(diesel::SqliteConnection);

#[catch(404)]
pub fn book_not_found() -> Value {
    json!("Book Not Found . . . ")
}

#[get("/")]
pub fn get_books(_auth: BasicAuth, _db: DbConn) -> Value {
    json!(
    [
        {"id":1, "name":"Horus Rising", "category":"science fiction"},
        {"id":2, "name":"Designing data intensive applications", "category":"computer science"}
        ]
    )
}

#[get("/<id>")]
pub fn get_book_by_id(id: i32, _auth: BasicAuth) -> Value {
    json!(
        {"id":id, "name":"always the same id?", "category":"always has been"}
    )
}

#[post("/", format = "json")]
pub fn create_book(_auth: BasicAuth) -> Value {
    json!(
        {"id":3, "name":"Filgrim", "category":"science fiction"}
    )
}

#[put("/<id>", format = "json")]
pub fn update_book(id: i32, _auth: BasicAuth) -> Value {
    json!(
        {"id":id, "name":"Updated", "category":"jokes on yee!"}
    )
}

#[delete("/<id>")]
pub fn delete_book(id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}
