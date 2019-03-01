use rocket::http::Status;
use rocket::request;
use rocket::request::FromRequest;
use rocket::Outcome;
use rocket::Request;

use crate::db::models::schema::instance;
use crate::db::models::AsJsonError;
use crate::db::models::ModelAs;
use crate::db::DatabaseConnection;
use diesel::prelude::*;
use std::error;
use std::rc::Rc;

use crate::db::models::schema::instance::dsl as instance_dsl;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Queryable, Identifiable, Clone)]
#[table_name = "instance"]
pub struct InstanceModel {
	pub id: i32,
	pub uuid: String,
	pub name: String,
	pub description: Option<String>,
	pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceJson {
	pub id: i32,
	pub uuid: String,
	pub name: String,
	pub description: Option<String>,
	pub enabled: bool,
}

pub struct Instance {
	pub model: Rc<InstanceModel>,
}

impl Instance {
	pub fn new(conn: &DatabaseConnection, id: i32) -> Result<Self> {
		if let Ok(model) = instance_dsl::instance
			.find(id)
			.first::<InstanceModel>(&conn.0)
		{
			return Ok(Instance {
				model: Rc::new(model),
			});
		}

		Err(Box::new(AsJsonError::new("Unable to read user by id")))
	}

	pub fn new_by_uuid(conn: &DatabaseConnection, uuid: &str) -> Result<Self> {
		if let Ok(model) = instance_dsl::instance
			.filter(instance_dsl::uuid.eq(uuid))
			.first::<InstanceModel>(&conn.0)
		{
			return Ok(Instance {
				model: Rc::new(model),
			});
		}

		Err(Box::new(AsJsonError::new("Unable to read user by uuid")))
	}
}

impl<'de> ModelAs<'de> for Instance {
	type OutputJson = InstanceJson;
	type OutputModel = InstanceModel;

	fn as_model(&self) -> Rc<Self::OutputModel> {
		Rc::clone(&self.model)
	}
}

impl From<Rc<InstanceModel>> for InstanceJson {
	fn from(model: Rc<InstanceModel>) -> Self {
		let model = model.as_ref().clone();

		InstanceJson {
			id: model.id,
			uuid: model.uuid,
			name: model.name,
			description: model.description,
			enabled: model.enabled,
		}
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for Instance {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
		match request.headers().get_one("X-Instance-UUID") {
			Some(uuid) => {
				let conn: DatabaseConnection = DatabaseConnection::from_request(request)?;

				if let Ok(instance) = Instance::new_by_uuid(&conn, uuid) {
					Outcome::Success(instance)
				} else {
					Outcome::Forward(())
				}
			}
			None => Outcome::Failure((Status::BadRequest, ())),
		}
	}
}
