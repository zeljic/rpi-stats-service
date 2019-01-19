pub mod schema;

use self::schema::*;

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "instance"]
pub struct InstanceModel {
	pub id: i32,
	pub uuid: String,
	pub name: String,
	pub description: Option<String>,
	pub enabled: bool,
}

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "log"]
pub struct LogModel {
	pub id: i32,
	pub instance_id: i32,
	pub log_type_id: i32,
	pub date_time: i32,
	pub value: f32,
	pub enabled: bool,
}

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "log_type"]
pub struct LogTypeModel {
	pub id: i32,
	pub name: String,
	pub enabled: bool,
}

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "mesh"]
pub struct MeshModel {
	pub id: i32,
	pub name: String,
	pub description: Option<String>,
	pub enabled: bool,
}

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "mesh_instance"]
pub struct MeshInstanceModel {
	pub id: i32,
	pub mesh_id: i32,
	pub instance_id: i32,
	pub enabled: bool,
}

#[derive(Debug, Queryable, Identifiable, Clone)]
#[table_name = "user"]
pub struct User {
	pub id: i32,
	pub name: Option<String>,
	pub email: Option<String>,
	pub password: Option<String>,
	pub enabled: bool,
}
