use db::models::CRUD;
use db::pool::PoolWrapper;
use rocket::http::Status;
use rocket::request;
use rocket::request::FromRequest;
use rocket::Outcome;
use rocket::Request;
use rusqlite::Connection;
use rusqlite::Error;
use rusqlite::Row;
use rusqlite::Statement;

#[derive(Serialize, Deserialize, Debug)]
pub struct Instance {
	pub id: Option<u32>,
	pub uuid: String,
	pub name: String,
	pub description: String,
	pub enabled: bool,
}

impl CRUD for Instance {
	type Output = Self;

	fn read(conn: &Connection, id: u32) -> Option<Self::Output> {
		let mut statement: Statement = conn
			.prepare("select * from `instance` where `id` = ?;")
			.expect("Unable to prepare statement");

		let mut rows = statement.query(&[&id]).expect("Unable to execute query");

		if let Some(result_row) = rows.next() {
			Some(Instance::from(result_row.expect("Unable to get row")))
		} else {
			None
		}
	}

	fn create(&self, conn: &Connection) -> Result<usize, Error> {
		let mut statement = conn
			.prepare("insert into `instance` (`uuid`, `name`, `description`) values (?, ?, ?);")
			.expect("Unable to prepare statement");

		statement.execute(&[&self.uuid, &self.name, &self.description])
	}
}

impl Instance {
	fn get_by_uuid(conn: &Connection, uuid: &str) -> Option<Self> {
		let mut statement: Statement = conn
			.prepare("select * from `instance` where uuid = ?;")
			.expect("Unable to prepare statement");

		let mut rows = statement.query(&[&uuid]).expect("Unable to execute query");

		if let Some(result_row) = rows.next() {
			Some(Instance::from(result_row.expect("Unable to get row")))
		} else {
			None
		}
	}
}

impl<'a, 'stmt> From<Row<'a, 'stmt>> for Instance {
	fn from(row: Row) -> Self {
		Instance {
			id: row.get("id"),
			uuid: row.get("uuid"),
			name: row.get("name"),
			description: row.get("description"),
			enabled: row.get("enabled"),
		}
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for Instance {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Instance, ()> {
		match request.headers().get_one("X-Instance-UUID") {
			Some(uuid) => {
				let pool_wrapper = PoolWrapper::from_request(request);

				match Instance::get_by_uuid(
					&*(pool_wrapper.expect("Unable to get connection pool")),
					uuid,
				) {
					Some(instance) => Outcome::Success(instance),
					None => Outcome::Failure((Status::BadRequest, ())),
				}
			}
			None => Outcome::Failure((Status::BadRequest, ())),
		}
	}
}
