#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use diesel::sqlite::SqliteConnection;
use models::Instance;

mod db;
mod models;
mod schema;

fn main() {
	println!("Zdravo, collector!");
}
