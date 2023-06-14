use diesel::result::Error::NotFound;
use rocket::serde::json::{json, Json, Value};
use rocket::{
    http::Status,
    response::status::{self, Custom},
};

use crate::models::Book;
use crate::{auth::BasicAuth, book_repo::BookRepo, models::NewBook};
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
pub async fn get_book_by_id(id: i32, _auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |conn| {
        BookRepo::find_by_id(conn, id)
            .map(|book| json!(book))
            .map_err(|err| match err {
                NotFound => Custom(Status::NotFound, json!(err.to_string())),
                _ => Custom(Status::InternalServerError, json!(err.to_string())),
            })
    })
    .await
}

#[post("/", format = "json", data = "<new_book>")]
pub async fn create_book(
    _auth: BasicAuth,
    db: DbConn,
    new_book: Json<NewBook>,
) -> Result<Value, Custom<Value>> {
    db.run(|conn| {
        BookRepo::create(conn, new_book.into_inner())
            .map(|book| json!(book))
            .map_err(|err| Custom(Status::InternalServerError, json!(err.to_string())))
    })
    .await
}

#[put("/<id>", format = "json", data = "<book>")]
pub async fn update_book(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
    book: Json<Book>,
) -> Result<Value, Custom<Value>> {
    db.run(move |con| {
        BookRepo::save(con, id, book.into_inner())
            .map(|book| json!(book))
            .map_err(|err| Custom(Status::InternalServerError, json!(err.to_string())))
    })
    .await
}

#[delete("/<id>")]
pub async fn delete_book(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |conn| {
        BookRepo::delete(conn, id)
            .map(|_| status::NoContent)
            .map_err(|err| Custom(Status::InternalServerError, json!(err.to_string())))
    })
    .await
}
