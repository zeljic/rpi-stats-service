use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{self, sqlite::SqliteConnection};
use schema::{self, instance, log, log_type};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "instance"]
pub struct Instance {
	pub id: Option<i32>,
	pub uuid: String,
	pub name: String,
	pub enabled: bool,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "log_type"]
pub struct LogType {
	pub id: Option<i32>,
	pub name: String,
	pub enabled: bool,
}

#[derive(Queryable, Insertable, Associations, Serialize, Deserialize, Debug)]
#[belongs_to(Instance)]
#[belongs_to(LogType)]
#[table_name = "log"]
pub struct Log {
	pub id: Option<i32>,
	pub instance_id: i32,
	pub log_type_id: i32,
	pub date_time: i32,
	pub value: f32,
}
