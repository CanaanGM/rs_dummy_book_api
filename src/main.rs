#[macro_use]
extern crate rocket;

use rocket::config::Config;
use rocket::response::status;
use rocket::serde::json::{json, Value};

#[get("/")]
fn handle_root() -> Value {
    json!("Greetings~!\n")
}

#[catch(404)]
fn book_not_found() -> Value {
    json!("Book Not Found . . . ")
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not Found . . . ")
}

#[get("/")]
fn get_books() -> Value {
    json!(
    [
        {"id":1, "name":"Horus Rising", "category":"science fiction"},
        {"id":2, "name":"Designing data intensive applications", "category":"computer science"}
        ]
    )
}

#[get("/<id>")]
fn get_book_by_id(id: i32) -> Value {
    json!(
        {"id":id, "name":"always the same id?", "category":"always has been"}
    )
}

#[post("/", format = "json")]
fn create_book() -> Value {
    json!(
        {"id":3, "name":"Filgrim", "category":"science fiction"}
    )
}

#[put("/<id>", format = "json")]
fn update_book(id: i32) -> Value {
    json!(
        {"id":id, "name":"Updated", "category":"jokes on yee!"}
    )
}

#[delete("/<id>")]
fn delete_book(id: i32) -> status::NoContent {
    status::NoContent
}

#[rocket::main]
async fn main() {
    let port = 9789;

    let config = Config {
        port: port,
        ..Config::default()
    };

    let _ = rocket::custom(config)
        .mount("/", routes![handle_root])
        .register("/", catchers![not_found]) // default catcher
        .mount(
            "/books/",
            routes![
                get_books,
                get_book_by_id,
                update_book,
                delete_book,
                create_book
            ],
        )
        .register("/books", catchers![book_not_found]) // custom catcher
        .launch()
        .await;
}
