use dotenv::dotenv;
use rocket_db_pools::Database;
use std::env;

mod db;
mod models;
mod schema;

use rocket::{get, launch, routes, Rocket};

#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

#[get("/home")]
fn home() -> &'static str {
	"This is home"
}
#[launch]
fn rocket() -> _ {
	rocket::build()
		.attach(db::Db::init())
		.mount("/", routes![index])
}

// fn main() {
// 	rocket().launch();
// }
