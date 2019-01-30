use crate::db::models::user::generate_password;
use crate::db::models::user::UserJson;
use crate::db::DatabaseConnection;
use rocket::Route;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

use crate::db::dmodels::schema::user::dsl as user_dsl;
use crate::db::models::user::User;
use crate::db::models::user::UserModel;
use crate::db::models::ModelAs;
use diesel::prelude::*;

#[get("/", format = "application/json")]
pub fn profile(user: User) -> JsonValue {
	if let Ok(user) = user.as_json() {
		return json!({
			"status": true,
			"user": user
		});
	}

	json!({
		"status": false
	})
}

#[post("/", format = "application/json", data = "<user_json>")]
pub fn profile_update(
	conn: DatabaseConnection,
	user: User,
	user_json: Json<UserJson>,
) -> JsonValue {
	let model = user.as_model();

	if let Ok(user) = user_dsl::user
		.filter(user_dsl::id.eq(model.id))
		.first::<UserModel>(&conn.0)
	{
		let result = diesel::update(&user)
			.set((
				user_dsl::email.eq(&user_json.email),
				user_dsl::name.eq(&user_json.name),
			))
			.execute(&conn.0);

		if result.is_ok() {
			if let Ok(user) = User::new(&conn, model.id) {
				match user.as_json() {
					Ok(json) => {
						return json!({
							"status": true,
							"user": json
						});
					}
					Err(_) => {
						return json!({
							"status": false
						});
					}
				}
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

#[post("/change-password", format = "json", data = "<new_password>")]
pub fn profile_change_password(
	conn: DatabaseConnection,
	user: User,
	new_password: Json<NewPassword>,
) -> JsonValue {
	if new_password.new != new_password.new_again {
		return json!({
			"status": false,
			"reason": "There is no password match"
		});
	}

	let user_model = user.as_model();

	if user_model.password.clone().unwrap() == generate_password(new_password.old.as_str()) {
		if let Ok(user) = user_dsl::user
			.filter(user_dsl::id.eq(user_model.id))
			.first::<UserModel>(&conn.0)
		{
			let update_result = diesel::update(&user)
				.set(user_dsl::password.eq(generate_password(new_password.new.as_str())))
				.execute(&conn.0);

			if update_result.is_ok() {
				if let Ok(user) = User::new(&conn, user_model.id) {
					match user.as_json() {
						Ok(json) => {
							return json!({
								"status": true,
								"user": json
							});
						}
						Err(_) => {
							return json!({
								"status": false
							});
						}
					}
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
