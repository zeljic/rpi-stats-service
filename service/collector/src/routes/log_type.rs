use crate::db::models::log_type::{LogType, LogTypeModel};
use crate::db::models::user::User;
use crate::db::DatabaseConnection;
use rocket::Route;
use rocket_codegen::routes;
use rocket_contrib::json::{Json, JsonValue};

use crate::db::models::ModelAs;

use crate::db::models::log_type::LogTypeJson;
use diesel::prelude::*;

use crate::db::models::schema::log_type::dsl as log_type_dsl;
use crate::error::ErrorKind;

#[get("/", format = "application/json")]
pub fn list(conn: DatabaseConnection, user: User) -> crate::error::Result<JsonValue> {
	let list = log_type_dsl::log_type
		.filter(log_type_dsl::user_id.eq(user.get_id()))
		.get_results::<LogTypeModel>(&conn.0)?
		.into_iter()
		.map(std::convert::Into::into)
		.collect::<Vec<LogTypeJson>>();

	Ok(json!({
	"status": true,
	"list": list,
	"count": list.len()
	}))
}

#[get("/<id>", format = "application/json")]
pub fn get(conn: DatabaseConnection, user: User, id: i32) -> crate::error::Result<JsonValue> {
	let item: LogTypeJson = log_type_dsl::log_type
		.filter(log_type_dsl::id.eq(id))
		.filter(log_type_dsl::user_id.eq(user.get_id()))
		.first::<LogTypeModel>(&conn.0)?
		.into();

	Ok(json!({
		"status": true,
		"item": item
	}))
}

#[post("/", format = "application/json", data = "<create_request>")]
pub fn create(
	conn: DatabaseConnection,
	user: User,
	create_request: Json<LogTypeJson>,
) -> crate::error::Result<JsonValue> {
	let mut create_request = create_request.into_inner();

	create_request.user_id.get_or_insert(user.get_id());

	let item: LogTypeJson = diesel::insert_into(log_type_dsl::log_type)
		.values(&create_request)
		.get_result::<LogTypeModel>(&conn.0)?
		.into();

	Ok(json!({
		"status": true,
		"item": item
	}))
}

#[put("/<id>", format = "application/json", data = "<update_request>")]
pub fn update(
	conn: DatabaseConnection,
	user: User,
	update_request: Json<LogTypeJson>,
	id: i32,
) -> crate::error::Result<JsonValue> {
	let log_type_model = LogType::new(&conn, id)?.as_model();

	if log_type_model.user_id != user.get_id() {
		return Err(ErrorKind::AccessDenied.into());
	}

	let item: LogTypeJson = diesel::update(log_type_model.as_ref())
		.set(update_request.into_inner())
		.get_result::<LogTypeModel>(&conn.0)?
		.into();

	Ok(json!({
		"status": true,
		"item": item
	}))
}

#[delete("/<id>")]
pub fn delete(conn: DatabaseConnection, user: User, id: i32) -> crate::error::Result<JsonValue> {
	let log_type_model = LogType::new(&conn, id)?.as_model();

	if log_type_model.user_id != user.get_id() {
		return Err(ErrorKind::AccessDenied.into());
	}

	diesel::delete(log_type_dsl::log_type)
		.filter(log_type_dsl::id.eq(id))
		.execute(&conn.0)?;

	Ok(json!({
		"status": true
	}))
}

pub fn get_routes() -> Vec<Route> {
	return routes![list, get, create, update, delete];
}
