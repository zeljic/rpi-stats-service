use crate::db::models::user::User;
use crate::db::models::user::UserModel;
use crate::db::models::ModelAs;
use crate::db::DatabaseConnection;
use rocket::Route;
use rocket_contrib::json::JsonValue;

use crate::db::models::mesh::Mesh;
use crate::db::models::mesh::MeshJson;
use crate::db::models::user_mesh::UserMeshModel;
use diesel::prelude::*;

#[get("/", format = "application/json")]
pub fn get(conn: DatabaseConnection, user: User) -> JsonValue {
	let model: UserModel = user.as_model().as_ref().clone();

	if let Ok(list) = UserMeshModel::belonging_to(&model).load::<UserMeshModel>(&conn.0) {
		println!("{:?}", &list);

		let list = list
			.iter()
			.filter_map(|item| {
				if let Ok(item) = Mesh::new(&conn, item.mesh_id) {
					return Some(item);
				} else {
					return None;
				}
			})
			.filter_map(|item| {
				if let Ok(item) = item.as_json() {
					return Some(item);
				} else {
					return None;
				}
			})
			.collect::<Vec<MeshJson>>();

		return json!({
			"status": true,
			"count": list.len(),
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
