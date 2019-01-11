use crate::db::models::CRUD;
use crate::db::pool::PoolWrapper;
use crate::routes::auth::Token;
use crate::session_manager::SessionManager;
use crate::session_manager::StoreItem;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rocket::request;
use rocket::request::FromRequest;
use rocket::{Outcome, Request, State};
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

		if let Some(result_row) = rows.next() {
			Some(User::from(result_row.expect("Unable to get row")))
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
		let token = Token::from_request(request).unwrap();
		let session_manager = request.guard::<State<RwLock<SessionManager>>>();
		let pool_wrapper = PoolWrapper::from_request(request);

		match session_manager {
			Outcome::Success(session_manager) => match session_manager.read() {
				Ok(session_manager) => match session_manager.get_session_token(&token) {
					Some(session) => match session.store.get("user_id") {
						Some(StoreItem::UserId(id)) => {
							let connection: PoolWrapper = pool_wrapper.unwrap();

							match User::read(&connection, *id as u32) {
								Some(user) => Outcome::Success(user),
								None => Outcome::Forward(()),
							}
						}
						_ => Outcome::Forward(()),
					},
					_ => Outcome::Forward(()),
				},
				_ => Outcome::Forward(()),
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
									Ok(mut write_guard) => {
										let token = Token::new();

										match write_guard.add_new_session_str(&token.key) {
											Some(session) => {
												session.add_to_store_str(
													"user_id",
													StoreItem::UserId(id),
												);

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
			enabled: row.get("enabled"),
		}
	}
}

#[inline(always)]
pub fn generate_password(input: &str) -> String {
	let mut sha256 = Sha256::new();

	sha256.input_str(input);

	sha256.result_str()
}
