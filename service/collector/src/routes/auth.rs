use crate::db::models::user::User;
use crate::db::DatabaseConnection;
use crate::session::session_manager::SessionManager;
use crate::session::token::Token;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use std::sync::RwLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
	email: Option<String>,
	password: Option<String>,
}

#[post("/", format = "application/json", data = "<login_request>")]
pub fn login(
	connection: DatabaseConnection,
	login_request: Json<LoginRequest>,
	session_manager: State<RwLock<SessionManager>>,
) -> JsonValue {
	let login_request = login_request.clone();

	let email = login_request.email.unwrap_or_default();
	let password = login_request.password.unwrap_or_default();

	match User::login(&connection, session_manager.inner(), &email, &password) {
		Some(token) => json!({
			"status": true,
			"token": &token.key
		}),
		None => json!({
			"status": true,
			"reason": "Invalid username and password"
		}),
	}
}

#[get("/", format = "application/json")]
pub fn logged(token: Token) -> JsonValue {
	return json!({
		"status": true,
		"logged": token.is_logged()
	});
}

#[get("/logout")]
pub fn logout(token: Token, session_manager: State<RwLock<SessionManager>>) -> JsonValue {
	if let Ok(mut session_manager) = session_manager.write() {
		if let Some(index) = session_manager.get_session_index_token(&token) {
			session_manager.sessions.remove(index);

			return json!({
				"status": true
			});
		}
	}

	json!({
		"status": false,
		"message": "Unknown error"
	})
}

pub fn get_routes() -> Vec<rocket::Route> {
	routes![login, logout, logged]
}
