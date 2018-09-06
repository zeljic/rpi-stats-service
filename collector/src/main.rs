#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate commons;
extern crate rocket;

use commons::db::init_pool;

#[get("/")]
fn index() -> &'static str {
	"Zdravo web svete"
}

fn main() {
	let rocket = rocket::ignite();

	rocket
		.manage(init_pool())
		.mount("/", routes![index])
		.launch();
}
