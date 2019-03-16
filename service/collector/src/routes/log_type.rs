use crate::db::models::log_type::LogTypeModel;
use crate::db::models::user::User;
use crate::db::DatabaseConnection;
use rocket::Route;
use rocket_codegen::routes;
use rocket_contrib::json::JsonValue;

use crate::db::models::ModelAs;

use crate::db::models::log_type::LogTypeJson;
use diesel::prelude::*;

#[get("/", format = "application/json")]
pub fn get(conn: DatabaseConnection, user: User) -> JsonValue {
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

pub fn get_routes() -> Vec<Route> {
	return routes![get];
}
