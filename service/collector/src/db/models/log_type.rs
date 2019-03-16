use crate::db::models::schema::log_type;
use crate::db::models::ModelAs;
use crate::db::DatabaseConnection;
use std::error;
use std::rc::Rc;

use crate::db::models::schema::log_type::dsl as log_type_dsl;
use crate::db::models::AsJsonError;
use diesel::prelude::*;

use crate::db::models::user::UserModel;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Queryable, Identifiable, Associations, PartialEq, Clone)]
#[table_name = "log_type"]
#[belongs_to(UserModel, foreign_key = "user_id")]
pub struct LogTypeModel {
	pub id: i32,
	pub user_id: i32,
	pub name: String,
	pub description: Option<String>,
	pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LogTypeJson {
	pub id: i32,
	pub user_id: i32,
	pub name: String,
	pub description: Option<String>,
	pub enabled: bool,
}

#[derive(Debug)]
pub struct LogType {
	model: Rc<LogTypeModel>,
}

impl LogType {
	pub fn new(conn: &DatabaseConnection, id: i32) -> Result<Self> {
		if let Ok(model) = log_type_dsl::log_type
			.find(id)
			.first::<LogTypeModel>(&conn.0)
		{
			return Ok(LogType {
				model: Rc::new(model),
			});
		}

		Err(Box::new(AsJsonError::new("Unable to read log_type by id")))
	}
}

impl<'de> ModelAs<'de> for LogType {
	type OutputJson = LogTypeJson;
	type OutputModel = LogTypeModel;

	fn as_model(&self) -> Rc<Self::OutputModel> {
		Rc::clone(&self.model)
	}
}

impl From<Rc<LogTypeModel>> for LogTypeJson {
	fn from(rc_model: Rc<LogTypeModel>) -> Self {
		rc_model.as_ref().clone().into()
	}
}

impl From<LogTypeModel> for LogTypeJson {
	fn from(model: LogTypeModel) -> Self {
		Self {
			id: model.id,
			user_id: model.user_id,
			name: model.name,
			description: model.description,
			enabled: model.enabled,
		}
	}
}
