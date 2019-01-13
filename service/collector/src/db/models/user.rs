use crate::db::models::CRUD;
use crate::db::pool::PoolWrapper;
use crate::session::session::Session;
use crate::session::session::SessionItem;
use crate::session::session_manager::SessionManager;
use crate::session::token::Token;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rocket::request;
use rocket::request::FromRequest;
use rocket::{Outcome, Request};
use rusqlite::{Connection, Error, Row, Statement};
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct User {
	#[serde(skip_serializing)]
	pub id: Option<u32>,
	pub name: String,
	pub email: String,
	#[serde(skip_serializing, skip_deserializing)]
	pub password: String,
	#[serde(skip_serializing, skip_deserializing)]
	pub enabled: bool,
}

impl CRUD for User {
	type Output = Self;

	fn read(conn: &Connection, id: u32) -> Option<<Self as CRUD>::Output> {
		let mut statement: Statement = conn
			.prepare("select * from `user` where `id` = ?;")
			.expect("Unable to prepare statement");

		let mut rows = statement.query(&[&id]).expect("Unable to execute query");

		if let Some(Ok(row)) = rows.next() {
			Some(User::from(row))
		} else {
			None
		}
	}

	fn create(&self, _conn: &Connection) -> Result<usize, Error> {
		unimplemented!()
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
		let connection: PoolWrapper = PoolWrapper::from_request(request)?;
		let session = Session::from_request(request)?;

		match session.store.get("user_id") {
			Some(SessionItem::UserId(user_id)) => match User::read(&connection, *user_id) {
				Some(user) => Outcome::Success(user),
				None => Outcome::Forward(()),
			},
			_ => Outcome::Forward(()),
		}
	}
}

impl User {
	pub fn login(
		conn: &Connection,
		session_manager: &RwLock<SessionManager>,
		email: &str,
		password: &str,
	) -> Option<Token> {
		match conn
			.prepare("select id from user where email = ? and password = ? and enabled = true;")
		{
			Ok(mut stmt) => {
				let hashed_password: String = generate_password(password);

				match stmt.query(&[&email, hashed_password.as_str()]) {
					Ok(mut rows) => match rows.next() {
						Some(row) => match row {
							Ok(row) => match row.get_checked::<&str, u32>("id") {
								Ok(id) => match session_manager.write() {
									Ok(mut session_write_guard) => {
										let token = Token::new();

										match session_write_guard.add_new_session_str(&token.key) {
											Some(session) => {
												session.add_to_store_str(
													"user_id",
													SessionItem::UserId(id),
												);

												session.ping();

												Some(token)
											}
											None => None,
										}
									}
									Err(_) => None,
								},
								Err(_) => None,
							},
							Err(_) => None,
						},
						None => None,
					},
					Err(_) => None,
				}
			}
			Err(_) => None,
		}
	}
}

impl<'a, 'stmt> From<Row<'a, 'stmt>> for User {
	fn from(row: Row<'a, 'stmt>) -> Self {
		User {
			id: row.get("id"),
			name: row.get("name"),
			email: row.get("email"),
			password: row.get("password"),
			enabled: row.get_checked::<&str, bool>("enabled").unwrap(),
		}
	}
}

#[inline(always)]
pub fn generate_password(input: &str) -> String {
	let mut sha256 = Sha256::new();

	sha256.input_str(input);

	sha256.result_str()
}
