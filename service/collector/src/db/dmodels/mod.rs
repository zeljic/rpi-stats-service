pub mod schema;

use self::schema::*;

#[derive(Debug, Queryable, Identifiable, Clone)]
#[table_name = "mesh_instance"]
pub struct MeshInstanceModel {
	pub id: i32,
	pub mesh_id: i32,
	pub instance_id: i32,
	pub enabled: bool,
}
