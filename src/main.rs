use db::Db;
use rocket_db_pools::Database;

mod db;
mod models;
mod routes;
mod schema;

use rocket::{
	fs::{relative, FileServer},
	launch, routes,
};
use routes::{create_post, get_posts, home};

#[launch]
fn rocket() -> _ {
	rocket::build()
		.attach(Db::init())
		.mount("/images", FileServer::from(relative!("images")))
		.mount("/avatars", FileServer::from(relative!("avatars")))
		.mount("/api", routes![create_post, get_posts])
		.mount("/", routes![home])
}

// fn main() {
// 	rocket().launch();
// }
