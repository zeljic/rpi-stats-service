use crate::db::models::log_type::{LogType, LogTypeModel};
use crate::db::models::user::User;
use crate::db::DatabaseConnection;
use rocket::Route;
use rocket_codegen::routes;
use rocket_contrib::json::{Json, JsonValue};

use crate::db::models::ModelAs;

use crate::db::models::AsJsonError;

use crate::db::models::log_type::LogTypeJson;
use diesel::prelude::*;

use crate::db::models::schema::log_type::dsl as log_type_dsl;
use std::error::Error;

#[get("/", format = "application/json")]
pub fn list(conn: DatabaseConnection, user: User) -> Result<JsonValue, Box<dyn Error>> {
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
pub fn get(conn: DatabaseConnection, user: User, id: i32) -> Result<JsonValue, Box<dyn Error>> {
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
) -> JsonValue {
	let mut create_request: LogTypeJson = create_request.into_inner();

	create_request.user_id.get_or_insert(user.get_id());

	match diesel::insert_into(log_type_dsl::log_type)
		.values(&create_request)
		.execute(&conn.0)
	{
		Ok(_) => json!({
			"status": true
		}),
		Err(_) => json!({
			"status": false
		}),
	}
}

#[put("/<id>", format = "application/json", data = "<update_request>")]
pub fn update(
	conn: DatabaseConnection,
	user: User,
	update_request: Json<LogTypeJson>,
	id: i32,
) -> Result<JsonValue, Box<dyn Error>> {
	let update_request = &update_request.into_inner();

	let log_type_model = LogType::new(&conn, id)?.as_model();

	if log_type_model.user_id != user.get_id() {
		return Err(Box::new(AsJsonError::new("Security issue")));
	}

	let item: LogTypeJson = diesel::update(log_type_model.as_ref())
		.set(update_request)
		.get_result::<LogTypeModel>(&conn.0)?
		.into();

	Ok(json!({
		"status": true,
		"item": item
	}))
}

#[delete("/<id>")]
pub fn delete(conn: DatabaseConnection, user: User, id: i32) -> Result<JsonValue, AsJsonError> {
	let log_type = LogType::new(&conn, id as i32);

	match log_type {
		Ok(log_type) => {
			if !log_type.is_user_id(&user) {
				return Err("Security issue".into());
			}

			match diesel::delete(log_type_dsl::log_type.filter(log_type_dsl::id.eq(id as i32)))
				.execute(&conn.0)
			{
				Ok(_size) => Ok(json!({
					"status": true
				})),
				Err(e) => Err(e.description().into()),
			}
		}
		Err(e) => Err(e.description().into()),
	}
}

pub fn get_routes() -> Vec<Route> {
	return routes![list, get, create, update, delete];
}
