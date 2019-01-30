use crate::db::dmodels::schema::log;
use std::error;

use crate::db::models::ModelAs;
use std::rc::Rc;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Queryable, Identifiable, Clone)]
#[table_name = "log"]
pub struct LogModel {
	pub id: i32,
	pub instance_id: i32,
	pub log_type_id: i32,
	pub date_time: i32,
	pub value: f32,
	pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogJson {}

pub struct Log {
	pub model: Rc<LogModel>,
}

impl<'de> ModelAs<'de> for Log {
	type OutputJson = LogJson;
	type OutputModel = LogModel;

	fn as_model(&self) -> Rc<Self::OutputModel> {
		Rc::clone(&self.model)
	}
}

impl From<Rc<LogModel>> for LogJson {
	fn from(_model: Rc<LogModel>) -> Self {
		unimplemented!()
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogCreateRequest {
	pub log_type_name: String,
	#[serde(skip_serializing, skip_deserializing)]
	pub log_type_id: u32,
	pub value: f64,
	pub date_time: u32,
}
