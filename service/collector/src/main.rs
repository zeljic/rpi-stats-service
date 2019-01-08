#![feature(proc_macro_hygiene, decl_macro, plugin)]
#![allow(dead_code)]
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_sqlite;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rusqlite;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate crypto;
extern crate rand;
extern crate serde_json;

use crate::db::pool::init_pool;
use crate::session_manager::init_rwlock_session_manager;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

mod db;
mod routes;
mod session_manager;

#[catch(404)]
fn error_404() -> Json<JsonValue> {
	Json(json!({
		"status": false,
		"reason": "Handler Not Found: Unable to handle this request"
	}))
}

#[catch(400)]
fn error_400() -> Json<JsonValue> {
	Json(json!({
		"status": false,
		"reason": "Bad Request: Unable to handle this request"
	}))
}

fn main() {
	rocket::ignite()
		.manage(init_pool())
		.manage(init_rwlock_session_manager())
		.mount("/", routes::basic::get_routes())
		.mount("/api/auth", routes::auth::get_routes())
		.mount("/api/user", routes::user::get_routes())
		.register(catchers![error_400, error_404])
		.launch();
}
