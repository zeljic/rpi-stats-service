use crate::routes::auth::Token;
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum StoreItem {
	LastAction(usize),
	UserId(u32),
}

pub struct Session {
	pub token: Token,
	pub store: HashMap<String, StoreItem>,
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

		if let Ok(timestamp) = SystemTime::now().duration_since(UNIX_EPOCH) {
			session.add_to_store_str(
				"last_action",
				StoreItem::LastAction(timestamp.as_secs() as usize),
			);
		}

		session
	}

	pub fn add_to_store_str(&mut self, key: &str, value: StoreItem) {
		self.add_to_store(String::from(key), value);
	}

	pub fn add_to_store(&mut self, key: String, value: StoreItem) {
		self.store.insert(key, value);
	}
}

pub struct SessionManager {
	pub sessions: Vec<Session>,
}

impl SessionManager {
	pub fn new() -> Self {
		let sessions = Vec::new();

		SessionManager { sessions }
	}

	pub fn get_session_str(&self, token: String) -> Option<&Session> {
		let sessions = &self.sessions;

		sessions.iter().find(|session| session.token == token)
	}

	pub fn get_session_str_mut(&mut self, token: &str) -> Option<&mut Session> {
		let sesions = &mut self.sessions;

		sesions.iter_mut().find(|session| session.token == token)
	}

	pub fn get_session_token(&self, token: &Token) -> Option<&Session> {
		let sessions = &self.sessions;

		sessions.iter().find(|session| token == &session.token)
	}

	pub fn get_session_index_token(&self, token: &Token) -> Option<usize> {
		let sessions = &self.sessions;

		sessions.iter().position(|session| session.token == token)
	}

	pub fn add_new_session_str(&mut self, token: &str) -> Option<&mut Session> {
		self.sessions.push(Session::new_str(String::from(token)));

		self.get_session_str_mut(token)
	}

	pub fn add_new_session_token(&mut self, token: Token) {
		self.sessions.push(Session::new_token(token));
	}
}

pub fn init_session_manager() -> SessionManager {
	SessionManager::new()
}

pub fn init_rwlock_session_manager() -> RwLock<SessionManager> {
	RwLock::new(SessionManager::new())
}
