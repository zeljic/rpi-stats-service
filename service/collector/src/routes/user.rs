use crate::db::lmodels::logical_user::generate_password;
use crate::db::lmodels::logical_user::LogicalUser;
use crate::db::lmodels::CRUD;
use crate::db::DatabaseConnection;
use rocket::Route;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

use crate::db::dmodels::schema::user::dsl as user_dsl;
use crate::db::dmodels::User;
use diesel::prelude::*;

#[get("/", format = "application/json")]
pub fn profile(user: LogicalUser) -> JsonValue {
	json!({
		"status": true,
		"user": user
	})
}

#[post("/", format = "application/json", data = "<json_user>")]
pub fn profile_update(
	conn: DatabaseConnection,
	user: LogicalUser,
	json_user: Json<LogicalUser>,
) -> JsonValue {
	if let Some(id) = user.id {
		if let Ok(user) = user_dsl::user
			.filter(user_dsl::id.eq(id))
			.first::<User>(&conn.0)
		{
			let result = diesel::update(&user)
				.set((
					user_dsl::email.eq(&json_user.email),
					user_dsl::name.eq(&json_user.name),
				))
				.execute(&conn.0);

			if result.is_ok() {
				let user: LogicalUser = user.into();

				return json!({
					"status": true,
					"user": user
				});
			}
		}
	}

	json!({
		"status": false
	})
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPassword {
	old: String,
	new: String,
	#[serde(rename = "newAgain")]
	new_again: String,
}

#[post("/change-password", format = "json", data = "<data>")]
pub fn profile_change_password(
	conn: DatabaseConnection,
	logical_user: LogicalUser,
	data: Json<NewPassword>,
) -> JsonValue {
	if data.new != data.new_again {
		return json!({
			"status": false,
			"reason": "There is no password match"
		});
	}

	if let Some(id) = logical_user.id {
		if logical_user.password == generate_password(data.old.as_str()) {
			if let Ok(user) = user_dsl::user
				.filter(user_dsl::id.eq(id))
				.first::<User>(&conn.0)
			{
				let result = diesel::update(&user)
					.set(user_dsl::password.eq(generate_password(data.new.as_str())))
					.execute(&conn.0);

				if result.is_ok() {
					let user: LogicalUser = user.into();

					return json!({
						"status": true,
						"user": user
					});
				}
			}
		}
	}

	json!({
		"status": false
	})
}

pub fn get_routes() -> Vec<Route> {
	routes![profile, profile_update, profile_change_password]
}
