#[macro_use]
extern crate rocket;

use rocket::config::Config;

#[get("/")]
fn handle_root() -> &'static str {
    "Hello !!"
}

#[rocket::main]
async fn main() {
    let port = 9789; // Custom port number

    // Create a custom configuration with the desired port
    let config = Config {
        port: port,
        ..Config::default()
    };

    let _ = rocket::custom(config)
        .mount("/", routes![handle_root])
        .launch()
        .await;
}
