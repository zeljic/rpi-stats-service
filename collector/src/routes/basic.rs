use crate::session_manager::SessionManager;
use rocket::State;
use rocket_contrib::json::JsonValue;
use std::sync::RwLock;

#[get("/")]
pub fn index() -> &'static str {
	"Zdravo web svete"
}

pub fn get_routes() -> Vec<rocket::Route> {
	routes![index]
}
