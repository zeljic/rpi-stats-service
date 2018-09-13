use db::models::CRUD;
use rusqlite::Connection;
use rusqlite::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
	pub id: Option<i64>,
	pub instance_id: i64,
	pub log_type_id: i64,
	pub date_time: i64,
	pub value: f64,
}

impl CRUD for Log {
	type Output = Log;

	fn read(_conn: &Connection, _id: i64) -> Option<Self::Output> {
		unimplemented!()
	}

	fn create(&self, conn: &Connection) -> Result<usize, Error> {
		let mut statement = conn.prepare("insert into `log` (`instance_id`, `log_type_id`, `date_time`, `value`) values (?, ?, ?, ?);").expect("Unable to prepare statement");

		statement.execute(&[
			&self.instance_id,
			&self.log_type_id,
			&self.date_time,
			&self.value,
		])
	}
}
