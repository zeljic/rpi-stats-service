use crate::db::models::user::User;

use crate::db::DatabaseConnection;
use rocket::Route;
use rocket_contrib::json::JsonValue;

use crate::db::models::mesh::MeshJson;
use crate::db::models::mesh::MeshModel;

use diesel::prelude::*;
use diesel::sql_types::Integer;

use crate::error::Result;

#[get("/", format = "application/json")]
pub fn list(conn: DatabaseConnection, user: User) -> Result<JsonValue> {
	let list: Vec<MeshJson> = diesel::sql_query(
		"select * from mesh where id in (select mesh_id from user_mesh where user_id = $1)",
	)
	.bind::<Integer, _>(user.get_id())
	.get_results::<MeshModel>(&conn.0)?
	.into_iter()
	.map(std::convert::Into::into)
	.collect::<Vec<MeshJson>>();

	Ok(json!({
		"status": true,
		"list": list
	}))
}

pub fn get_routes() -> Vec<Route> {
	return routes![list];
}
