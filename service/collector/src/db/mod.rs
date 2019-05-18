pub mod models;

#[database("db")]
pub struct DatabaseConnection(diesel::PgConnection);
