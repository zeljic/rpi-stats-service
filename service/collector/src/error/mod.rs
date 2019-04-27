use failure::{Backtrace, Context, Fail};
use std::fmt::Display;
use std::io::Cursor;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Fail, Clone, Copy, Eq, PartialEq)]
pub enum ErrorKind {
	#[fail(display = "Access Denied error")]
	AccessDenied,

	#[fail(display = "Database error")]
	Database,

	#[fail(display = "Unknown error")]
	Unknown,
}

#[derive(Debug)]
pub struct Error {
	pub inner: Context<ErrorKind>,
}

impl Error {
	pub fn kind(&self) -> ErrorKind {
		*self.inner.get_context()
	}
}

impl Fail for Error {
	fn cause(&self) -> Option<&Fail> {
		self.inner.cause()
	}

	fn backtrace(&self) -> Option<&Backtrace> {
		self.inner.backtrace()
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		Display::fmt(&self.inner, f)
	}
}

impl From<ErrorKind> for Error {
	fn from(kind: ErrorKind) -> Self {
		Error {
			inner: Context::new(kind),
		}
	}
}

impl From<Context<ErrorKind>> for Error {
	fn from(context: Context<ErrorKind>) -> Self {
		Error { inner: context }
	}
}

impl From<diesel::result::Error> for Error {
	fn from(diesel_error: diesel::result::Error) -> Self {
		println!("{}", diesel_error);

		ErrorKind::Database.into()
	}
}

impl<'r> rocket::response::Responder<'r> for Error {
	fn respond_to(self, _: &rocket::request::Request) -> rocket::response::Result<'r> {
		let json: rocket_contrib::json::JsonValue = json!({
			"status": false,
			"reason": format!("{}", self.kind())
		});

		let json: serde_json::value::Value = json.0;

		rocket::response::Response::build()
			.sized_body(Cursor::new(json.to_string()))
			.header(rocket::http::ContentType::JSON)
			.ok()
	}
}
