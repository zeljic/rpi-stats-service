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

#[get("/", format = "application/json")]
pub fn list(conn: DatabaseConnection, user: User) -> JsonValue {
	let user_model = user.as_model().as_ref().clone();

	if let Ok(list) = LogTypeModel::belonging_to(&user_model).load::<LogTypeModel>(&conn.0) {
		let list = list
			.into_iter()
			.map(std::convert::Into::into)
			.collect::<Vec<LogTypeJson>>();

		return json!({
			"status": true,
			"list": list
		});
	}

	json!({
		"status": false
	})
}

#[post("/", format = "application/json", data = "<create_request>")]
pub fn create(
	conn: DatabaseConnection,
	user: User,
	create_request: Json<LogTypeJson>,
) -> JsonValue {
	let mut create_request = create_request.clone();

	let user_model = user.as_model().as_ref().clone();

	create_request.user_id.get_or_insert(user_model.id);

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

#[get("/<id>", format = "application/json")]
pub fn get(conn: DatabaseConnection, user: User, id: i32) -> JsonValue {
	if let Ok(item) = log_type_dsl::log_type
		.filter(log_type_dsl::id.eq(id))
		.filter(log_type_dsl::user_id.eq(user.as_model().id))
		.first::<LogTypeModel>(&conn.0)
	{
		return json!({
			"status": true,
			"item": LogTypeJson::from(item)
		});
	}

	json!({
		"status": true
	})
}

#[put("/", format = "application/json", data = "<update_request>")]
pub fn update(
	conn: DatabaseConnection,
	user: User,
	update_request: Json<LogTypeJson>,
) -> Result<JsonValue, AsJsonError> {
	let update_request = &update_request.into_inner();

	if let Ok(log_type) = LogType::from_json(&conn, update_request) {
		if !log_type.is_user_id(&user) {
			return Err(AsJsonError::new("Security issue"));
		}

		let log_type_model: LogTypeModel = log_type.as_model().as_ref().clone();

		match diesel::update(&log_type_model)
			.set(update_request)
			.execute(&conn.0)
		{
			Ok(_) => {
				return Ok(json!({
					"status": true
				}));
			}
			Err(_) => {
				return Ok(json!({
					"status": false
				}));
			}
		}
	}

	Ok(json!({
		"status": false
	}))
}

pub fn get_routes() -> Vec<Route> {
	return routes![list, get, create, update];
}
