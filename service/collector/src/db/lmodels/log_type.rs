#[derive(Serialize, Deserialize, Debug)]
pub struct LogType {
	pub id: Option<u32>,
	pub name: String,
	pub enabled: bool,
}

/*impl<'a, 'stmt> From<Row<'a, 'stmt>> for LogType {
	fn from(row: Row<'a, 'stmt>) -> Self {
		LogType {
			id: row.get("id"),
			name: row.get("name"),
			enabled: row.get("enabled"),
		}
	}
}*/

/*impl LogType {
	pub fn get_by_name(conn: &Connection, name: &str) -> Option<Self> {
		let mut stmt = conn
			.prepare("select * from log_type where name = ?")
			.expect("Unable to prepare statement");

		let mut rows = stmt.query(&[&name]).expect("Unable to execute query");

		if let Some(rows_result) = rows.next() {
			Some(Self::from(rows_result.expect("Unable to get row")))
		} else {
			None
		}
	}
}*/
