#![feature(proc_macro_hygiene, decl_macro, plugin)]

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
extern crate serde_json;

use crate::db::models::instance::Instance;
use crate::db::models::log::Log;
use crate::db::models::log::LogCreateRequest;
use crate::db::models::CRUD;
use crate::db::pool::init_pool;
use crate::db::pool::PoolWrapper;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

mod db;

#[get("/")]
fn index() -> &'static str {
	"Zdravo web svete"
}

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

#[post("/", format = "application/json", data = "<data>")]
fn log_create(conn: PoolWrapper, instance: Instance, data: Json<LogCreateRequest>) -> Json<JsonValue> {
	let log = Log::new(&*conn, &instance, &data.into_inner());

	match log {
		Ok(log) => match log.create(&*conn) {
			Ok(_) => Json(json!({
				"status": true
			})),
			Err(msg) => Json(json!({
				"status": false,
				"reason": format!("{:?}", msg)
			})),
		},
		Err(msg) => Json(json!({
				"status": false,
				"reason": msg
			})),
	}
}

fn main() {
	let pool = init_pool();

	rocket::ignite()
		.manage(pool)
		.mount("/", routes![index])
		.mount("/api/log", routes![log_create])
		.mount("/api/log_type", routes![])
		.mount("/api/instance", routes![])
		.register(catchers![error_400, error_404])
		.launch();
}
