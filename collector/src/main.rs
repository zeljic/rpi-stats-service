#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate commons;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use commons::db::init_pool;
use rocket_contrib::{Json, Value};

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
	let rocket = rocket::ignite();

	rocket
		.manage(init_pool())
		.mount("/", routes![index])
		.catch(catchers![error_404])
		.launch();
}
