use crate::routes::auth::Token;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rocket::request;
use rocket::request::FromRequest;
use rocket::Request;
use rusqlite::Connection;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
	pub id: Option<u32>,
	pub name: String,
	pub email: String,
	pub password: String,
	pub enabled: bool,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
	type Error = ();

	fn from_request(_request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
		unimplemented!()
	}
}

impl User {
	pub fn login(conn: &Connection, email: &str, password: &str) -> Option<Token> {
		let mut stmt = conn
			.prepare("select id from user where email = ? and password = ? and enabled = true")
			.expect("Unable to create statement");

		let hashed_password: String = generate_password(password);

		let mut rows = stmt
			.query(&[&email, hashed_password.as_str()])
			.expect("Unable to execute query");

		if rows.next().is_some() {
			Some(Token::new())
		} else {
			None
		}
	}
}

#[inline(always)]
fn generate_password(input: &str) -> String {
	let mut sha256 = Sha256::new();

	sha256.input_str(input);

	sha256.result_str()
}