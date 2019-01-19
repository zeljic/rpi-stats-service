use crate::db::DatabaseConnection;
use diesel::result::Error;

pub mod instance;
pub mod log;
pub mod log_type;
pub mod logical_user;
pub mod mesh;

pub trait CRUD {
	type Output;

	fn read(conn: &DatabaseConnection, id: i32) -> Option<Self::Output> {
		unimplemented!()
	}
	fn create(&self, conn: &DatabaseConnection) -> Result<(), Error> {
		unimplemented!()
	}
	fn delete(&self, conn: &DatabaseConnection) -> Result<(), Error> {
		unimplemented!()
	}
}

pub trait Enable {
	fn set_enabled(&self, conn: DatabaseConnection, state: bool) -> Result<(), Error> {
		unimplemented!()
	}
}
