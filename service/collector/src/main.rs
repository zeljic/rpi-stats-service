#![feature(proc_macro_hygiene, decl_macro)]
#![allow(dead_code)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

use crate::session::session_manager::init_rwlock_session_manager;
use rocket_contrib::json::JsonValue;

use crate::db::DatabaseConnection;

mod db;
mod routes;
mod session;

#[catch(404)]
fn error_404() -> JsonValue {
	json!({
		"status": false,
		"reason": "Handler Not Found: Unable to handle this request"
	})
}

#[catch(400)]
fn error_400() -> JsonValue {
	json!({
		"status": false,
		"reason": "Bad Request: Unable to handle this request"
	})
}

fn main() {
	rocket::ignite()
		.attach(DatabaseConnection::fairing())
		.manage(init_rwlock_session_manager())
		.mount("/", routes::basic::get_routes())
		.mount("/api/auth", routes::auth::get_routes())
		.mount("/api/user", routes::user::get_routes())
		.mount("/api/instance", routes::instance::get_routes())
		.mount("/api/mesh", routes::mesh::get_routes())
		.mount("/api/log-type", routes::log_type::get_routes())
		.register(catchers![error_400, error_404])
		.launch();
}
