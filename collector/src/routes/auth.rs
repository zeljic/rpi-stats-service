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
pub struct Token(pub String);

impl Token {
	pub fn new() -> Self {
		let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(256).collect();

		let mut sha256 = Sha256::new();
		sha256.input_str(&rand_string);

		Token(sha256.result_str())
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
		match request.headers().get_one("X-Token") {
			Some(token) => match request.guard::<State<RwLock<SessionManager>>>() {
				Outcome::Success(session_manager) => {
					let session_manager_read = session_manager.read();

					match session_manager_read {
						Ok(read_guard) => {
							let token = Token(String::from(token));

							match read_guard.get_session_token(&token) {
								Some(ref _session) => Outcome::Success(token),
								None => Outcome::Failure((Status::BadRequest, ())),
							}
						}
						Err(_) => Outcome::Failure((Status::BadRequest, ())),
					}
				}
				Outcome::Failure(_) => Outcome::Failure((Status::BadRequest, ())),
				Outcome::Forward(_) => Outcome::Failure((Status::BadRequest, ())),
			},
			None => Outcome::Failure((Status::BadRequest, ())),
		}
	}
}

impl PartialEq for Token {
	fn eq(&self, other: &Token) -> bool {
		self.0 == other.0
	}
}

impl PartialEq<String> for Token {
	fn eq(&self, other: &String) -> bool {
		self.0 == *other
	}
}

impl PartialEq<&Token> for Token {
	fn eq(&self, other: &&Token) -> bool {
		self.0 == other.0
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
		&login_request.email,
		&login_request.password,
	) {
		None => json!({
				"status": false,
				"reason": "Invalid username and password"
			}),
		Some(token) => {
			let mut session_manager = session_manager
				.write()
				.expect("Unable to get session manager");

			session_manager.add_new_session_str(&token.0);

			json!({
				"status": true,
				"token": &token.0
			})
		}
	}
}

#[get("/logout")]
pub fn logout(token: Token, session_manager: State<RwLock<SessionManager>>) -> &'static str {
	if let Ok(mut session_manager) = session_manager.write() {
		if let Some(index) = session_manager.get_session_index_token(&token) {
			session_manager.sessions.remove(index);
		}
	}

	""
}

pub fn get_routes() -> Vec<rocket::Route> {
	routes![login, logout]
}
