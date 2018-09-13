use db::models::CRUD;
use rusqlite::Connection;
use rusqlite::Error;
use rusqlite::Statement;

#[derive(Serialize, Deserialize, Debug)]
pub struct Instance {
	pub id: Option<i64>,
	pub uuid: String,
	pub name: String,
	pub enabled: bool,
}

impl CRUD for Instance {
	type Output = Self;

	fn read(conn: &Connection, id: i64) -> Option<Self::Output> {
		let mut statement: Statement = conn
			.prepare("select * from `instance` where `id` = ?;")
			.expect("Unable to prepare statement");

		let mut rows = statement.query(&[&id]).expect("Unable to execute query");

		if let Some(result_row) = rows.next() {
			let row = result_row.expect("Unable to get row");

			Some(Instance {
				id: row.get("id"),
				uuid: row.get("uuid"),
				name: row.get("name"),
				enabled: row.get("enabled"),
			})
		} else {
			None
		}
	}

	fn create(&self, conn: &Connection) -> Result<usize, Error> {
		let mut statement = conn
			.prepare("insert into `instance` (`uuid`, `name`) values (?, ?);")
			.expect("Unable to prepare statement");

		statement.execute(&[&self.uuid, &self.name])
	}
}
