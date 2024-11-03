use std::io::Write;

use crate::db::Db;
use crate::models::{Blogpost, InsertableBlogpost};
use anyhow::anyhow;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::{get, http::ContentType, post, Data};
use rocket_db_pools::Connection;
use rocket_multipart_form_data::{
	mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};
use serde_json::{json, Value};
enum ImageType {
	Avatar,
	PostImage,
}
fn save_image(img_type: ImageType, bytes: &[u8]) -> Result<String, anyhow::Error> {
	let file_uuid = uuid::Uuid::new_v4();
	let file_path = match img_type {
		ImageType::PostImage => format!("./images/{}.png", file_uuid),
		ImageType::Avatar => format!("./avatars/{}.png", file_uuid),
	};
	println!("Processing the image");
	// let image_name = Uuid
	let file =
		std::fs::File::create(&file_path).map_err(|e| format!("File creation error: {:?}", e));
	if file.is_err() {
		return Err(anyhow!("image file is corrupted"));
	}
	let file_result = file
		.unwrap()
		.write_all(&bytes)
		.map_err(|e| format!("File saving error: {:?}", e));
	if file_result.is_err() {
		println!("{}", file_result.unwrap_err());
	}
	Ok(file_path)
}

#[get("/home")]
pub async fn home() -> Result<NamedFile, std::io::Error> {
	NamedFile::open("./static/home.html").await
}

async fn process_avatar(avatar_url: &Option<String>) -> Result<Option<String>, anyhow::Error> {
	match avatar_url {
		Some(url) => {
			// let parsed_url = Url
			let response_result = reqwest::get(url).await;
			if response_result.is_err() {
				return Err(anyhow!("Failed to get a response from avatar url"));
			}
			let response = response_result.unwrap();
			if !response.status().is_success() {
				return Err(anyhow!("Response is not a success"));
			}
			if match response.headers().get("content-type") {
				Some(header) => header.to_str().unwrap_or(""),
				None => "",
			} != "image/png"
			{
				return Err(anyhow!("Invalid filetype"));
			}
			let image_bytes = response.bytes().await;
			if image_bytes.is_err() {
				return Err(anyhow!("Error processing bytes"));
			}
			let image_saved = save_image(ImageType::Avatar, &(image_bytes.unwrap()));
			if image_saved.is_err() {
				return Err(image_saved.unwrap_err());
			}
			Ok(Some(image_saved.unwrap()))
		}
		None => Ok(None),
	}
}
#[post("/create_post", data = "<data>")]
pub async fn create_post(
	content_type: &ContentType,
	db: Connection<Db>,
	data: Data<'_>,
) -> Json<Value> {
	println!("Got here");
	// Define form fields
	let mut options = MultipartFormDataOptions::new();
	options.allowed_fields.extend(vec![
		MultipartFormDataField::text("avatar_url"),
		MultipartFormDataField::text("username"),
		MultipartFormDataField::text("message"),
		MultipartFormDataField::raw("image")
			.size_limit(40 * 1024 * 1024) // Limit to 5 MB
			.content_type_by_string(Some(mime::IMAGE_STAR))
			.expect("Failed to set content type for image"),
	]);

	// Parse the form data
	let multipart_form = MultipartFormData::parse(content_type, data, options)
		.await
		.map_err(|e| format!("Form parsing error: {:?}", e));
	if multipart_form.is_err() {
		println!("{}", multipart_form.unwrap_err());
		return Json(json!({"error":"error processing the form data"}));
	}

	// // Extract text fields
	let mut multipart_form_data = multipart_form.unwrap();
	let username = match multipart_form_data
		.texts
		.remove("username")
		.and_then(|texts| texts.into_iter().next())
		.map(|t| t.text)
	{
		Some(x) => x,
		None => return Json(json!({"error":"no username in form"})),
	};
	let mut avatar_url = multipart_form_data
		.texts
		.remove("avatar_url")
		.and_then(|texts| texts.into_iter().next())
		.map(|t| t.text);
	let message = match multipart_form_data
		.texts
		.remove("message")
		.and_then(|texts| texts.into_iter().next())
		.map(|t| t.text)
	{
		Some(x) => x,
		None => return Json(json!({"error":"no body in form"})),
	};
	avatar_url = match avatar_url {
		Some(av) => {
			if av == "" {
				None
			} else {
				Some(av)
			}
		}
		None => None,
	};
	println!("{:?}", avatar_url);
	let avatar_filepath: Option<String> = match process_avatar(&avatar_url).await {
		Ok(file_uuid) => file_uuid,
		Err(error) => return Json(json!({"error":format!("{:?}",error)})),
	};
	println!("{:?}", avatar_filepath);
	// Extract and save the uploaded file
	let mut file_path = None;
	if let Some(image_field) = multipart_form_data
		.raw
		.remove("image")
		.and_then(|mut files| files.pop())
	{
		let saved_image = save_image(ImageType::PostImage, &image_field.raw);
		if saved_image.is_err() {
			return Json(json!({"error":format!("{:?}",saved_image.unwrap_err())}));
		}
		file_path = Some(saved_image.unwrap());
	}
	let post = InsertableBlogpost {
		username,
		body: message,
		avatar_path: avatar_filepath,
		image_path: file_path,
	};
	let insert_result = post.insert(db).await;
	if insert_result.is_err() {
		println!("{}", insert_result.unwrap_err());
		return Json(json!({"error":"failed to insert post to database"}));
	}
	// Ok(format!(
	// 	"Form submitted: name = {:?}, email = {:?}, subject = {:?}, message = {:?}",
	// 	name, email, subject, message
	// ))
	return Json(json!({"success":"true"}));
}

#[get("/get_posts")]
pub async fn get_posts(db: Connection<Db>) -> Json<Value> {
	let posts = Blogpost::get_all(db).await;
	if posts.is_err() {
		return Json(json!({"error":"failed to get posts"}));
	}
	Json(json!(posts.unwrap()))
}
