use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

use dotenv::dotenv;
use std::env;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool() -> SqlitePool {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

	let manager = ConnectionManager::<SqliteConnection>::new(database_url);

	Pool::new(manager).expect("Database Pool is not created")
}

pub struct PooledSqliteConnection(pub PooledConnection<ConnectionManager<SqliteConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for PooledSqliteConnection {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
		let pool = request.guard::<State<SqlitePool>>()?;

		match pool.get() {
			Ok(conn) => Outcome::Success(PooledSqliteConnection(conn)),
			Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
		}
	}
}

impl Deref for PooledSqliteConnection {
	type Target = SqliteConnection;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
