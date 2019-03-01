use crate::db::models::instance::InstanceModel;
use crate::db::models::mesh::MeshModel;
use crate::db::models::schema::mesh_instance;
use crate::db::models::ModelAs;
use std::rc::Rc;

#[derive(Debug, Queryable, Identifiable, Associations, PartialEq, Clone)]
#[table_name = "mesh_instance"]
#[belongs_to(MeshModel, foreign_key = "mesh_id")]
#[belongs_to(InstanceModel, foreign_key = "instance_id")]
pub struct MeshInstanceModel {
	pub id: i32,
	pub mesh_id: i32,
	pub instance_id: i32,
	pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeshInstanceJson {
	pub id: i32,
	pub mesh_id: i32,
	pub instance_id: i32,
	pub enabled: bool,
}

pub struct MeshInstance {
	model: Rc<MeshInstanceModel>,
}

impl<'de> ModelAs<'de> for MeshInstance {
	type OutputJson = MeshInstanceJson;
	type OutputModel = MeshInstanceModel;

	fn as_model(&self) -> Rc<Self::OutputModel> {
		Rc::clone(&self.model)
	}
}

impl From<Rc<MeshInstanceModel>> for MeshInstanceJson {
	fn from(_model: Rc<MeshInstanceModel>) -> Self {
		unimplemented!()
	}
}
