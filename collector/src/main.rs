#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rusqlite;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use db::models::instance::Instance;
use db::models::log::Log;
use db::models::CRUD;
use db::pool::init_pool;
use db::pool::PoolWrapper;
use rocket_contrib::{Json, Value};
use std::error::Error;

mod db;

#[get("/")]
fn index() -> &'static str {
	"Zdravo web svete"
}

#[catch(404)]
fn error_404() -> Json<Value> {
	Json(json!({
		"status": false,
		"reason": "Handler Not Found: Unable to handle this request"
	}))
}

#[catch(400)]
fn error_400() -> Json<Value> {
	Json(json!({
		"status": false,
		"reason": "Bad Request: Unable to handle this request"
	}))
}

#[post("/", format = "application/json", data = "<json_log>")]
fn log_create(instance: Instance, conn: PoolWrapper, json_log: Json<Log>) -> Json<Value> {
	println!("{:?}", instance);

	match json_log.into_inner().create(&*conn) {
		Ok(_log) => Json(json!({
				"status": true
			})),
		Err(e) => Json(json!({
				"status": false,
				"reason": e.description()
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
		.catch(catchers![error_400, error_404])
		.launch();
}
