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

use crate::db::models::schema::user;
use crate::db::models::schema::user::dsl as user_dsl;
use crate::db::models::AsJsonError;
use crate::db::models::ModelAs;
use diesel::prelude::*;
use std::error;
use std::rc::Rc;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Queryable, Identifiable, Clone)]
#[table_name = "user"]
pub struct UserModel {
	pub id: i32,
	pub name: Option<String>,
	pub email: Option<String>,
	pub password: Option<String>,
	pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserJson {
	pub id: Option<i32>,
	pub name: String,
	pub email: String,
	pub enabled: bool,
}

pub struct User {
	model: Rc<UserModel>,
}

impl User {
	pub fn new(conn: &DatabaseConnection, id: i32) -> Result<Self> {
		if let Ok(model) = user_dsl::user.find(id).first::<UserModel>(&conn.0) {
			return Ok(User {
				model: Rc::new(model),
			});
		}

		Err(Box::new(AsJsonError::new("Unable to read user by id")))
	}
}

impl<'de> ModelAs<'de> for User {
	type OutputJson = UserJson;
	type OutputModel = UserModel;

	fn as_model(&self) -> Rc<Self::OutputModel> {
		Rc::clone(&self.model)
	}
}

impl User {
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
			.first::<UserModel>(&conn.0)
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

	pub fn am_i_id(&self, id: Option<i32>) -> bool {
		if let Some(id) = id {
			id == self.model.id
		} else {
			false
		}
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
		let connection: DatabaseConnection = DatabaseConnection::from_request(request)?;
		let session = Session::from_request(request)?;

		match session.store.get("user_id") {
			Some(SessionItem::UserId(user_id)) => {
				if let Ok(user) = User::new(&connection, *user_id) {
					Outcome::Success(user)
				} else {
					Outcome::Forward(())
				}
			}
			_ => Outcome::Forward(()),
		}
	}
}

impl From<Rc<UserJson>> for UserJson {
	fn from(rc_user_json: Rc<UserJson>) -> Self {
		rc_user_json.as_ref().clone()
	}
}

impl From<Rc<UserModel>> for UserJson {
	fn from(user_model: Rc<UserModel>) -> Self {
		let model: UserModel = user_model.as_ref().clone();

		UserJson {
			id: Option::from(user_model.id),
			name: model.name.unwrap_or_else(|| String::from("")),
			email: model.email.unwrap_or_else(|| String::from("")),
			enabled: model.enabled,
		}
	}
}

#[inline(always)]
pub fn generate_password(input: &str) -> String {
	let mut sha256 = Sha256::new();

	sha256.input_str(input);

	sha256.result_str()
}
