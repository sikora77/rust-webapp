use rocket_db_pools::diesel::{prelude::*, PgPool};
use rocket_db_pools::{Connection, Database};

#[derive(Database)]
#[database("my_db")]
pub struct Db(PgPool);

// #[get("/")]
// async fn use_db(mut db: Connection<Db>) {
// 	println!("Hello world");
// }
