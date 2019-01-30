use crate::db::dmodels::schema::log_type;
use crate::db::models::ModelAs;
use std::rc::Rc;

#[derive(Debug, Queryable, Identifiable, Clone)]
#[table_name = "log_type"]
pub struct LogTypeModel {
	pub id: i32,
	pub name: String,
	pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogTypeJson {
	pub id: Option<u32>,
	pub name: String,
	pub enabled: bool,
}

pub struct LogType {
	model: Rc<LogTypeModel>,
}

impl<'de> ModelAs<'de> for LogType {
	type OutputJson = LogTypeJson;
	type OutputModel = LogTypeModel;

	fn as_model(&self) -> Rc<Self::OutputModel> {
		Rc::clone(&self.model)
	}
}

impl From<Rc<LogTypeModel>> for LogTypeJson {
	fn from(_model: Rc<LogTypeModel>) -> Self {
		unimplemented!()
	}
}
