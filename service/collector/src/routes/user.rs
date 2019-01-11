use crate::db::models::user::generate_password;
use crate::db::models::user::User;
use crate::db::models::CRUD;
use crate::db::pool::PoolWrapper;
use rocket::Route;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

#[get("/", format = "application/json")]
pub fn profile(user: User) -> JsonValue {
	json!({
		"status": true,
		"user": user
	})
}

#[post("/", format = "application/json", data = "<json_user>")]
pub fn profile_update(conn: PoolWrapper, user: User, json_user: Json<User>) -> JsonValue {
	match conn.prepare("update user set name = ?, email = ? where id = ?;") {
		Ok(mut stmt) => {
			match stmt.execute(&[
				&json_user.name,
				&json_user.email,
				&user.id.unwrap().to_string(),
			]) {
				Ok(_) => {
					let user = User::read(&conn, user.id.unwrap());

					json!({
						"status": true,
						"user": user
					})
				}
				Err(_) => json!({
					"status": false
				}),
			}
		}
		Err(_) => json!({
			"status": false
		}),
	}
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
	conn: PoolWrapper,
	user: User,
	data: Json<NewPassword>,
) -> JsonValue {
	if data.new != data.new_again {
		return json!({
			"status": false,
			"reason": "There is no password match"
		});
	}

	match user.id {
		Some(id) => match User::read(&conn, id) {
			Some(user) => {
				if user.password == generate_password(data.old.as_str()) {
					match conn.prepare("update user set password = ? where id = ?;") {
						Ok(mut stmt) => match stmt.execute(&[
							generate_password(data.new.as_str()).as_str(),
							&id.to_string(),
						]) {
							Ok(_) => json!({
								"status": true
							}),
							Err(_) => json!({
								"status": true,
								"reason": "Unable to update user's password"
							}),
						},
						Err(_) => json!({
							"status": true,
							"reason": "Unable to prepare statment"
						}),
					}
				} else {
					json!({
						"status": true,
						"reason": "Inavalid password"
					})
				}
			}
			None => json!({
				"status": true,
				"reason": "Unable to get user by id"
			}),
		},
		None => json!({
			"status": true,
			"reason": "Unable to get user by id"
		}),
	}
}

pub fn get_routes() -> Vec<Route> {
	routes![profile, profile_update, profile_change_password]
}
