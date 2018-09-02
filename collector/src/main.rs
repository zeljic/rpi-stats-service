#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate commons;
extern crate serde_json;

use commons::db::init_pool;
use commons::db::DbConn;

mod models;
mod schema;

#[get("/")]
fn index() -> &'static str {
	"Zdravo web svete"
}

fn main() {
	let rocket = rocket::ignite();

	rocket.manage(init_pool()).mount("/", routes![index]).launch();
}
