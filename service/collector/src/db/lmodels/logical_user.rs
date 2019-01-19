use crate::db::dmodels::User;
use crate::db::lmodels::CRUD;
use crate::db::DatabaseConnection;
use crate::session::session::Session;
use crate::session::session::SessionItem;
use crate::session::session_manager::SessionManager;
use crate::session::token::Token;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rocket::request;
use rocket::request::FromRequest;
use rocket::{Outcome, Request};
use std::sync::RwLock;

use crate::db::dmodels::schema::user::dsl as user_dsl;
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct LogicalUser {
	#[serde(skip_serializing)]
	pub id: Option<i32>,
	pub name: String,
	pub email: String,
	#[serde(skip_serializing, skip_deserializing)]
	pub password: String,
	#[serde(skip_serializing, skip_deserializing)]
	pub enabled: bool,
}

impl CRUD for LogicalUser {
	type Output = Self;

	fn read(conn: &DatabaseConnection, id: i32) -> Option<<Self as CRUD>::Output> {
		match user_dsl::user
			.filter(user_dsl::enabled.eq(true))
			.filter(user_dsl::id.eq(id))
			.first::<User>(&conn.0)
		{
			Ok(user) => Some(LogicalUser::from(user)),
			Err(_) => None,
		}
	}
}

impl LogicalUser {
	pub fn login(
		conn: &DatabaseConnection,
		session_manager: &RwLock<SessionManager>,
		email: &str,
		password: &str,
	) -> Option<Token> {
		let hashed_password: String = generate_password(password);

		match user_dsl::user
			.filter(user_dsl::email.eq(email))
			.filter(user_dsl::password.eq(hashed_password))
			.filter(user_dsl::enabled.eq(true))
			.first::<User>(&conn.0)
		{
			Ok(user) => match session_manager.write() {
				Ok(mut session_write_guard) => {
					let token = Token::new();

					match session_write_guard.add_new_session_str(&token.key) {
						Some(session) => {
							session.add_to_store_str("user_id", SessionItem::UserId(user.id));

							session.ping();

							Some(token)
						}
						None => None,
					}
				}
				Err(_) => None,
			},
			Err(_) => None,
		}
	}
}

impl From<User> for LogicalUser {
	fn from(user: User) -> Self {
		LogicalUser {
			id: Some(user.id),
			name: user.name.unwrap(),
			email: user.email.unwrap(),
			password: user.password.unwrap(),
			enabled: user.enabled,
		}
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for LogicalUser {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
		let connection: DatabaseConnection = DatabaseConnection::from_request(request)?;
		let session = Session::from_request(request)?;

		match session.store.get("user_id") {
			Some(SessionItem::UserId(user_id)) => match LogicalUser::read(&connection, *user_id) {
				Some(user) => Outcome::Success(user),
				None => Outcome::Forward(()),
			},
			_ => Outcome::Forward(()),
		}
	}
}

#[inline(always)]
pub fn generate_password(input: &str) -> String {
	let mut sha256 = Sha256::new();

	sha256.input_str(input);

	sha256.result_str()
}
