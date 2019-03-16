use crate::db::models::user::User;
use rocket_contrib::json::JsonValue;

use crate::db::models::user::UserModel;
use crate::db::models::user_mesh::UserMeshModel;
use crate::db::models::ModelAs;
use crate::db::DatabaseConnection;
use diesel::prelude::*;
use rocket::Route;

#[get("/", format = "application/json")]
pub fn get(conn: DatabaseConnection, user: User) -> JsonValue {
	let model: UserModel = user.as_model().as_ref().clone();

	if let Ok(list) = UserMeshModel::belonging_to(&model).load::<UserMeshModel>(&conn.0) {
		let list: Vec<UserMeshModel> = list;

		let _list = list.into_iter().map(|item| {
			return item;
		});
	}

	json!({
		"status": false
	})
}

pub fn get_routes() -> Vec<Route> {
	routes![get]
}
