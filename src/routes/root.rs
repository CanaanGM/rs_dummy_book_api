use serde_json::{json, Value};

#[get("/")]
pub fn handle_root() -> Value {
    json!("Greetings~!\n")
}

#[catch(404)]
pub fn not_found() -> Value {
    json!("Not Found . . . ")
}
