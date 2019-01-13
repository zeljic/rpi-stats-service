use crate::session::session_manager::SessionManager;
use crate::session::token::Token;
use rocket::request;
use rocket::request::FromRequest;
use rocket::Outcome;
use rocket::Request;
use rocket::State;
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub enum SessionItem {
	LastAction(Duration),
	UserId(u32),
}

#[derive(Clone)]
pub struct Session {
	pub token: Token,
	pub store: HashMap<String, SessionItem>,
}

impl Session {
	pub fn new_str(token: String) -> Self {
		Session::new_token(Token::from_string(token))
	}

	pub fn new_token(token: Token) -> Self {
		let mut session = Session {
			token,
			store: HashMap::new(),
		};

		session.ping();

		session
	}

	pub fn add_to_store_str(&mut self, key: &str, value: SessionItem) {
		self.add_to_store(String::from(key), value);
	}

	pub fn add_to_store(&mut self, key: String, value: SessionItem) {
		self.store.insert(key, value);
	}

	pub fn ping(&mut self) {
		if let Ok(timestamp) = SystemTime::now().duration_since(UNIX_EPOCH) {
			self.add_to_store_str("last_action", SessionItem::LastAction(timestamp));
		}
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for Session {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
		let token: Token = Token::from_request(request)?;
		let session_manager = request.guard::<State<RwLock<SessionManager>>>();

		match session_manager {
			Outcome::Success(session) => match session.read() {
				Ok(session) => match session.get_session_token(&token) {
					Some(session) => Outcome::Success(session.clone()),
					None => Outcome::Forward(()),
				},
				Err(_) => Outcome::Forward(()),
			},
			_ => Outcome::Forward(()),
		}
	}
}
