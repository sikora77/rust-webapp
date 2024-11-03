use crate::db;
use crate::schema::blogposts;
use crate::schema::blogposts::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::query_dsl::methods::SelectDsl;
use rocket_db_pools::{diesel::prelude::RunQueryDsl, Connection};
use serde::Serialize;

#[derive(Clone, Insertable, Queryable, Serialize)]
#[table_name = "blogposts"]
pub struct Blogpost {
	pub id: i32,
	pub body: String,
	pub publish_date: NaiveDateTime,
	pub username: String,
	pub avatar_path: Option<String>,
	pub image_path: Option<String>,
}

impl Blogpost {
	pub async fn get_by_id(
		post_id: i32,
		mut db: Connection<db::Db>,
	) -> Result<Option<Blogpost>, diesel::result::Error> {
		blogposts::table
			.filter(blogposts::id.eq(post_id))
			.first::<Blogpost>(&mut db)
			.await
			.optional()
	}
	pub async fn get_all(
		mut db: Connection<db::Db>,
	) -> Result<Vec<Blogpost>, diesel::result::Error> {
		SelectDsl::select(blogposts, blogposts::all_columns)
			.order(publish_date.desc())
			.get_results(&mut db)
			.await
	}
	pub async fn insert(&self, mut db: Connection<db::Db>) -> Result<bool, diesel::result::Error> {
		match self.insert_into(blogposts::table).execute(&mut db).await {
			Ok(_) => Ok(true),
			Err(err) => Err(err),
		}
	}
}

#[derive(Clone, Insertable)]
#[table_name = "blogposts"]
pub struct InsertableBlogpost {
	pub body: String,
	pub username: String,
	pub avatar_path: Option<String>,
	pub image_path: Option<String>,
}
impl InsertableBlogpost {
	pub async fn insert(&self, mut db: Connection<db::Db>) -> Result<bool, diesel::result::Error> {
		match self.insert_into(blogposts::table).execute(&mut db).await {
			Ok(_) => Ok(true),
			Err(err) => Err(err),
		}
	}
}
