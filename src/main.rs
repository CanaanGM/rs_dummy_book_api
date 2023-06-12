#[macro_use]
extern crate rocket;
mod auth;
mod routes;

use rocket::config::Config;
use routes::books::{
    book_not_found, create_book, delete_book, get_book_by_id, get_books, update_book,DbConn
};
use routes::root::{handle_root, not_found};

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
        .attach(DbConn::fairing()) // adding in the databse struct to rocket
        .launch()
        .await;
}
