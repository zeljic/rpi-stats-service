use serde::Deserialize;
use serde::Serialize;
use std::rc::Rc;

pub mod instance;
pub mod log;
pub mod log_type;
pub mod mesh;
pub mod mesh_instance;
pub mod schema;
pub mod user;
pub mod user_mesh;

use crate::error::Result;

pub trait ModelAs<'de> {
	type OutputJson: Serialize + Deserialize<'de> + From<Rc<Self::OutputModel>>;
	type OutputModel;

	fn as_model(&self) -> Rc<Self::OutputModel>;

	fn as_json(&self) -> Result<Self::OutputJson> {
		Ok(self.as_model().into())
	}
}
