use crate::db::models::user::User;
use rocket::Route;
use rocket_contrib::json::JsonValue;

#[get("/", format = "application/json")]
pub fn profile(user: User) -> JsonValue {
	json!({
		"status": true,
		"user": user
	})
}

pub fn get_routes() -> Vec<Route> {
	routes![profile]
}
