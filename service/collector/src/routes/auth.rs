use crate::db::models::user::User;
use crate::db::pool::PoolWrapper;
use crate::session::session_manager::SessionManager;
use crate::session::token::Token;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use std::sync::RwLock;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
	email: String,
	password: String,
}

#[post("/", format = "application/json", data = "<login_request>")]
pub fn login(
	connection_pool_wrapper: PoolWrapper,
	login_request: Json<LoginRequest>,
	session_manager: State<RwLock<SessionManager>>,
) -> JsonValue {
	match User::login(
		&connection_pool_wrapper,
		session_manager.inner(),
		&login_request.email,
		&login_request.password,
	) {
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
