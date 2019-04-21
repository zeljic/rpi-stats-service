use crate::db::models::user::User;
use rocket_contrib::json::JsonValue;

use crate::db::models::instance::{InstanceJson, InstanceModel};

use crate::db::DatabaseConnection;
use diesel::prelude::*;

use diesel::sql_types::Integer;
use rocket::Route;
use std::error::Error;

#[get("/", format = "application/json")]
pub fn list(conn: DatabaseConnection, user: User) -> Result<JsonValue, Box<dyn Error>> {
	let list = diesel::sql_query(
		r#"
		select * from instance where id in (
			select instance_id from mesh_instance where mesh_id in (
				select mesh_id from user_mesh where	user_id = $1
			)
		);
		"#,
	)
	.bind::<Integer, _>(user.get_id())
	.get_results::<InstanceModel>(&conn.0)?
	.into_iter()
	.map(std::convert::Into::into)
	.collect::<Vec<InstanceJson>>();

	Ok(json!({
	"status": true,
	"list": list,
	"count": list.len()
	}))
}

pub fn get_routes() -> Vec<Route> {
	routes![list]
}
