pub mod dmodels;
pub mod lmodels;

#[database("db")]
pub struct DatabaseConnection(pub diesel::SqliteConnection);
