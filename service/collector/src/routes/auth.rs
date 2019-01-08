use crate::db::models::user::User;
use crate::db::pool::PoolWrapper;
use crate::session_manager::SessionManager;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::http::Status;
use rocket::request;
use rocket::request::FromRequest;
use rocket::request::Request;
use rocket::{Outcome, State};
use rocket_contrib::json::{Json, JsonValue};
use std::sync::RwLock;

#[derive(Debug)]
pub struct Token {
	pub key: String,
	state: bool,
}

impl Token {
	pub fn new() -> Self {
		let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(256).collect();

		let mut sha256 = Sha256::new();
		sha256.input_str(&rand_string);

		Token::from_string(sha256.result_str())
	}

	pub fn from_string(key: String) -> Self {
		Token { key, state: false }
	}

	pub fn from_str(key: &str) -> Self {
		Token::from_string(String::from(key))
	}

	pub fn empty() -> Self {
		Token::from_string(String::new())
	}

	pub fn is_logged(&self) -> bool {
		self.state
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
		match request.headers().get_one("X-Token") {
			Some(token) => match request.guard::<State<RwLock<SessionManager>>>() {
				Outcome::Success(session_manager) => match session_manager.read() {
					Ok(read_guard) => {
						let mut token = Token::from_str(token);

						token.state = read_guard.get_session_token(&token).is_some();

						Outcome::Success(token)
					}
					Err(_) => Outcome::Failure((Status::BadRequest, ())),
				},
				_ => Outcome::Failure((Status::BadRequest, ())),
			},
			None => Outcome::Success(Token::empty()),
		}
	}
}

impl PartialEq for Token {
	fn eq(&self, other: &Token) -> bool {
		self.key == other.key
	}
}

impl PartialEq<String> for Token {
	fn eq(&self, other: &String) -> bool {
		self.key == *other
	}
}

impl PartialEq<&Token> for Token {
	fn eq(&self, other: &&Token) -> bool {
		self.key == other.key
	}
}

impl PartialEq<&str> for Token {
	fn eq(&self, other: &&str) -> bool {
		self.key == *other
	}
}

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
