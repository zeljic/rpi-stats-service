pub mod models;

#[database("db")]
pub struct DatabaseConnection(diesel::PgConnection);

impl DatabaseConnection {
	pub fn raw(&self) -> &diesel::PgConnection {
		&*self.0
	}
}
