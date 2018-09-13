#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rusqlite;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use db::pool::init_pool;
use rocket_contrib::{Json, Value};

mod db;

#[get("/")]
fn index() -> &'static str {
	"Zdravo web svete"
}

#[catch(404)]
fn error_404() -> Json<Value> {
	Json(json!({
		"status": false,
		"reason": "Unable to handle this request"
	}))
}

fn main() {
	let pool = init_pool();

	rocket::ignite()
		.manage(pool)
		.mount("/", routes![index])
		.mount("/api/log", routes![])
		.mount("/api/log_type", routes![])
		.mount("/api/instance", routes![])
		.catch(catchers![error_404])
		.launch();
}
