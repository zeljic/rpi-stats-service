use crate::db::DatabaseConnection;

use crate::db::dmodels::schema::mesh::dsl as mesh_dsl;
use diesel::prelude::*;
use std::rc::Rc;

use crate::db::dmodels::schema::mesh;
use crate::db::models::ModelAs;

use crate::db::models::AsJsonError;
use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Queryable, Identifiable, Clone)]
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
		if let Ok(model) = mesh_dsl::mesh
			.filter(mesh_dsl::id.eq(id))
			.first::<MeshModel>(&conn.0)
		{
			return Ok(Mesh {
				model: Rc::new(model),
			});
		}

		Err(Box::new(AsJsonError::new("Unable to read user by id")))
	}
}

impl<'de> ModelAs<'de> for Mesh {
	type OutputJson = MeshJson;
	type OutputModel = MeshModel;

	fn as_model(&self) -> Rc<Self::OutputModel> {
		Rc::clone(&self.model)
	}
}

impl From<Rc<MeshModel>> for MeshJson {
	fn from(rc_model: Rc<MeshModel>) -> Self {
		let model = rc_model.as_ref().clone();

		Self {
			id: Option::from(model.id),
			name: Option::from(model.name),
			description: model.description,
			enabled: Option::from(model.enabled),
		}
	}
}
