#[get("/")]
pub fn index() -> &'static str {
	"Zdravo web svete"
}

pub fn get_routes() -> Vec<rocket::Route> {
	routes![index]
}
