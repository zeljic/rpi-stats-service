pub mod models;

#[database("db")]
pub struct DatabaseConnection(pub diesel::SqliteConnection);
