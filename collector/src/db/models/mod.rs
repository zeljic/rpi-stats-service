use rusqlite::Connection;
use rusqlite::Error;

pub mod instance;
pub mod log;
pub mod log_type;
pub mod mesh;

pub trait CRUD {
	type Output;

	fn read(conn: &Connection, id: u32) -> Option<Self::Output>;
	fn create(&self, conn: &Connection) -> Result<usize, Error>;
}

pub trait Enable {
	fn set_enabled(&self, conn: &Connection, state: bool) -> Result<(), Error>;
}
