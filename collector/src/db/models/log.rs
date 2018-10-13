use db::models::instance::Instance;
use db::models::log_type::LogType;
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
	pub enabled: bool,
}

impl CRUD for Log {
	type Output = Log;

	fn read(_conn: &Connection, _id: i64) -> Option<Self::Output> {
		unimplemented!()
	}

	fn create(&self, conn: &Connection) -> Result<usize, Error> {
		let mut statement = conn.prepare("insert into `log` (`instance_id`, `log_type_id`, `date_time`, `value`, `enabled`) values (?, ?, ?, ?, ?);")?;

		statement.execute(&[
			&self.instance_id,
			&self.log_type_id,
			&self.date_time,
			&self.value,
			&self.enabled,
		])
	}
}

impl Log {
	pub fn new(
		conn: &Connection,
		instance: &Instance,
		log_create_request: &LogCreateRequest,
	) -> Result<Self, String> {
		let mut log = Log {
			id: None,
			instance_id: 0,
			log_type_id: 0,
			date_time: 0,
			value: 0.0,
			enabled: false,
		};

		match instance.id {
			Some(v) => {
				log.instance_id = v;
			}
			None => return Err(String::from("Unable to read instance id")),
		}

		match LogType::get_by_name(&*conn, &log_create_request.log_type_name) {
			Some(log_type) => match log_type.id {
				Some(id) => {
					log.log_type_id = id;
				}
				None => return Err(String::from("There isn't id")),
			},
			None => return Err(String::from("Unable to find log_type by log_type_name")),
		}

		log.value = log_create_request.value;
		log.date_time = log_create_request.date_time;

		log.enabled = true;

		Ok(log)
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogCreateRequest {
	pub log_type_name: String,
	pub value: f64,
	pub date_time: i64,
}
