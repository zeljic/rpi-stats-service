use crate::db::models::user::User;
use crate::db::DatabaseConnection;
use crate::session::session_manager::SessionManager;
use crate::session::token::Token;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use std::sync::RwLock;

use crate::error::{ErrorKind, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
	email: Option<String>,
	password: Option<String>,
}

#[post("/", format = "application/json", data = "<login_request>")]
pub fn login(
	conn: DatabaseConnection,
	login_request: Json<LoginRequest>,
	session_manager: State<RwLock<SessionManager>>,
) -> Result<JsonValue> {
	let login_request = login_request.clone();

	let email = login_request.email.unwrap_or_default();
	let password = login_request.password.unwrap_or_default();

	let token: Token = User::login(&conn, session_manager.inner(), &email, &password)?;

	Ok(json!({
		"status": true,
		"token": &token.key
	}))
}

#[get("/", format = "application/json")]
pub fn logged(token: Token) -> JsonValue {
	return json!({
		"status": true,
		"logged": token.is_logged()
	});
}

#[get("/logout")]
pub fn logout(token: Token, session_manager: State<RwLock<SessionManager>>) -> Result<JsonValue> {
	let mut session_manager = session_manager
		.try_write()
		.map_err(|_| ErrorKind::AccessDenied)?;

	let index = session_manager
		.get_session_index_token(&token)
		.ok_or_else(|| ErrorKind::AccessDenied)?;

	session_manager.sessions.remove(index);

	Ok(json!({
		"status": true
	}))
}

pub fn get_routes() -> Vec<rocket::Route> {
	routes![login, logout, logged]
}
