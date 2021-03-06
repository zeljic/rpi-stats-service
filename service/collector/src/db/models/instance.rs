use rocket::{http::Status, request, request::FromRequest, Outcome, Request};

use crate::db::models::schema::instance;
use crate::db::models::ModelAs;
use crate::db::DatabaseConnection;
use crate::error::Result;
use diesel::prelude::*;
use std::rc::Rc;

use crate::db::models::schema::instance::dsl as instance_dsl;

#[derive(Debug, Queryable, Identifiable, Clone, QueryableByName)]
#[table_name = "instance"]
pub struct InstanceModel {
	pub id: i32,
	pub user_id: i32,
	pub uuid: String,
	pub name: String,
	pub description: Option<String>,
	pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Insertable, AsChangeset)]
#[table_name = "instance"]
pub struct InstanceJson {
	pub id: Option<i32>,
	pub user_id: Option<i32>,
	pub uuid: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub enabled: Option<bool>,
}

pub struct Instance {
	pub model: Rc<InstanceModel>,
}

impl Instance {
	pub fn new(conn: &DatabaseConnection, id: i32) -> Result<Self> {
		let model = instance_dsl::instance
			.find(id)
			.first::<InstanceModel>(conn.raw())?;

		Ok(Self {
			model: Rc::new(model),
		})
	}

	pub fn new_by_uuid(conn: &DatabaseConnection, uuid: &str) -> Result<Self> {
		let model = instance_dsl::instance
			.filter(instance_dsl::uuid.eq(uuid))
			.first::<InstanceModel>(conn.raw())?;

		Ok(Instance {
			model: Rc::new(model),
		})
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
	fn from(rc_model: Rc<InstanceModel>) -> Self {
		rc_model.as_ref().clone().into()
	}
}

impl From<InstanceModel> for InstanceJson {
	fn from(instance_model: InstanceModel) -> Self {
		Self {
			id: Option::from(instance_model.id),
			user_id: Option::from(instance_model.user_id),
			uuid: Option::from(instance_model.uuid),
			name: Option::from(instance_model.name),
			description: instance_model.description,
			enabled: Option::from(instance_model.enabled),
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
