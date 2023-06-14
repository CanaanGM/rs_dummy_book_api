use diesel::prelude::*;
use rocket::{
    http::Status,
    response::status::{self, Custom},
};
use serde_json::{json, Value};

use crate::{auth::BasicAuth, book_repo::BookRepo, models::Book, schema::books};
use rocket_sync_db_pools::database;

extern crate diesel;

#[database("sqlite")]
pub struct DbConn(diesel::SqliteConnection);

#[catch(404)]
pub fn book_not_found() -> Value {
    json!("Book Not Found . . . ")
}

#[get("/")]
pub async fn get_books(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|connection| {
        BookRepo::find_multiple(connection, 50)
            .map(|b| json!(b))
            .map_err(|err| Custom(Status::InternalServerError, json!(err.to_string())))
    })
    .await
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
