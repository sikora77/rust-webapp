use crate::schema::blogposts;
use crate::schema::blogposts::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Clone, Insertable)]
#[table_name = "blogposts"]
struct Blogpost {
	pub id: i32,
	pub body: String,
	pub publish_date: NaiveDateTime,
}
