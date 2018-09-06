use diesel;
use diesel::prelude::*;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response;
use rocket::response::Responder;
use rocket::Request;
use rocket::Response;
use schema::{instance, log, log_type};
use std::io::Cursor;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[table_name = "instance"]
#[primary_key(id)]
pub struct Instance {
	pub id: i32,
	pub uuid: String,
	pub name: String,
	pub enabled: bool,
}

#[derive(Insertable)]
#[table_name = "instance"]
pub struct NewInstance {
	pub uuid: String,
	pub name: String,
	pub enabled: bool,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[table_name = "log_type"]
#[primary_key(id)]
pub struct LogType {
	pub id: i32,
	pub name: String,
	pub enabled: bool,
}

#[derive(Insertable)]
#[table_name = "log_type"]
pub struct NewLogType {
	pub name: String,
	pub enabled: bool,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Debug)]
#[belongs_to(Instance)]
#[belongs_to(LogType)]
#[table_name = "log"]
#[primary_key(id)]
pub struct Log {
	pub id: i32,
	pub instance_id: i32,
	pub log_type_id: i32,
	pub date_time: i32,
	pub value: f32,
}

#[derive(Insertable)]
#[table_name = "log"]
pub struct NewLog {
	pub instance_id: i32,
	pub log_type_id: i32,
	pub date_time: i32,
	pub value: f32,
}
