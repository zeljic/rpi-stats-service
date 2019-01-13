use crate::routes::auth::Token;
use crate::session::session::Session;
use std::sync::RwLock;

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

	pub fn get_session_token_mut(&mut self, token: &Token) -> Option<&mut Session> {
		let sesions = &mut self.sessions;

		sesions.iter_mut().find(|session| session.token == token)
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
