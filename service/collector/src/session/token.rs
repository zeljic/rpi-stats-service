use crate::session::session_manager::SessionManager;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::request;
use rocket::request::FromRequest;
use rocket::request::Request;
use rocket::{Outcome, State};
use std::sync::RwLock;

#[derive(Debug, Clone)]
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
					Err(_) => Outcome::Forward(()),
				},
				_ => Outcome::Forward(()),
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
