use crate::db::DatabaseConnection;

use crate::db::models::schema::mesh::dsl as mesh_dsl;
use diesel::prelude::*;
use std::rc::Rc;

use crate::db::models::schema::mesh;
use crate::db::models::ModelAs;

use crate::error::Result;

#[derive(Debug, Queryable, Identifiable, Clone, QueryableByName)]
#[table_name = "mesh"]
pub struct MeshModel {
	pub id: i32,
	pub name: String,
	pub description: Option<String>,
	pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MeshJson {
	id: Option<i32>,
	name: Option<String>,
	description: Option<String>,
	enabled: Option<bool>,
}

#[derive(Debug)]
pub struct Mesh {
	model: Rc<MeshModel>,
}

impl Mesh {
	pub fn new(conn: &DatabaseConnection, id: i32) -> Result<Self> {
		let model = mesh_dsl::mesh.find(id).first::<MeshModel>(conn.raw())?;

		return Ok(Mesh {
			model: Rc::new(model),
		});
	}
}

impl<'de> ModelAs<'de> for Mesh {
	type OutputJson = MeshJson;
	type OutputModel = MeshModel;

	fn as_model(&self) -> Rc<Self::OutputModel> {
		Rc::clone(&self.model)
	}
}

impl From<MeshModel> for MeshJson {
	fn from(model: MeshModel) -> Self {
		Self {
			id: Option::from(model.id),
			name: Option::from(model.name),
			description: model.description,
			enabled: Option::from(model.enabled),
		}
	}
}

impl From<Rc<MeshModel>> for MeshJson {
	fn from(rc_model: Rc<MeshModel>) -> Self {
		rc_model.as_ref().clone().into()
	}
}
