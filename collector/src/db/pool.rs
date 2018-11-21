use dotenv::dotenv;
use r2d2::Pool;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rocket::http::Status;
use rocket::request;
use rocket::request::FromRequest;
use rocket::Outcome;
use rocket::{Request, State};
use rusqlite::Connection;
use std::env;
use std::ops::Deref;

pub fn init_pool() -> Pool<SqliteConnectionManager> {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

	let manager = SqliteConnectionManager::file(database_url);

	Pool::builder()
		.max_size(8)
		.build(manager)
		.expect("Connection pool is not created")
}

pub struct PoolWrapper(pub PooledConnection<SqliteConnectionManager>);

impl<'a, 'r> FromRequest<'a, 'r> for PoolWrapper {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
		let pool = request
			.guard::<State<Pool<SqliteConnectionManager>>>()
			.expect("Unable to get <State<Pool<SqliteConnectionManager>>>");

		match pool.get() {
			Ok(pooled_connection) => Outcome::Success(PoolWrapper(pooled_connection)),
			Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
		}
	}
}

impl Deref for PoolWrapper {
	type Target = Connection;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.0
	}
}
