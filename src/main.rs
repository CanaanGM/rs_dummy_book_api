#[macro_use]
extern crate rocket;
mod auth;
mod routes;
use rocket::fairing::AdHoc;
use rocket::{Rocket, Build};

use rocket::config::Config;
use routes::books::{
    book_not_found, create_book, delete_book, get_book_by_id, get_books, update_book,DbConn
};
use routes::root::{handle_root, not_found};


async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    DbConn::get_one(&rocket)
        .await
        .expect("Unable to retrieve connection").run(|c| {
            c.run_pending_migrations(MIGRATIONS).expect("Migrations failed");
        })
        .await;

    rocket
}

#[rocket::main]
async fn main() {
    let port = 9789;

    let config = Config {
        port: port,
        ..Config::default()
    };

    let _ = rocket::build()//  custom(config)
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
        .attach(AdHoc::on_ignite("Diesel migrations", run_db_migrations))
        .launch()
        .await;
}
