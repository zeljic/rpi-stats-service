use std::rc::Rc;

use crate::db::models::mesh::MeshModel;
use crate::db::models::schema::user_mesh;
use crate::db::models::user::UserModel;
use crate::db::models::ModelAs;

#[derive(Debug, Queryable, Identifiable, Associations, PartialEq, Clone)]
#[table_name = "user_mesh"]
#[belongs_to(MeshModel, foreign_key = "mesh_id")]
#[belongs_to(UserModel, foreign_key = "user_id")]
pub struct UserMeshModel {
	pub id: i32,
	pub user_id: i32,
	pub mesh_id: i32,
	pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserMeshJson {
	pub id: i32,
	pub user_id: i32,
	pub mesh_id: i32,
	pub enabled: bool,
}

pub struct UserMesh {
	model: Rc<UserMeshModel>,
}

impl<'de> ModelAs<'de> for UserModel {
	type OutputJson = UserMeshJson;
	type OutputModel = UserMeshModel;

	fn as_model(&self) -> Rc<Self::OutputModel> {
		unimplemented!()
	}
}

impl From<Rc<UserMeshModel>> for UserMeshJson {
	fn from(_model: Rc<UserMeshModel>) -> Self {
		unimplemented!()
	}
}
