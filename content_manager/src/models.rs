use crate::schema::*;

#[derive(Queryable, Deserialize, Debug)]
pub struct Content {
	pub content_id: i32,
	pub site_id: i32, // references site
	pub inner_path: String,
	pub size: i32,
	pub size_files: i32,
	pub size_files_optional: i32,
	pub modified: i32,
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name = "content"]
pub struct NewContent {
	pub site_id: i32, // references site
	pub inner_path: String,
	pub size: i32,
	pub size_files: i32,
	pub size_files_optional: i32,
	pub modified: i32,
}

impl From<zerucontent::Content> for NewContent {
	fn from(content: zerucontent::Content) -> NewContent {
		let size_fold = |p: usize, (_, file):  (&String, &zerucontent::File)| p + file.size;
		let size_files = content.files.iter().fold(0, size_fold) as i32;
		let size_files_optional = content.files.iter().fold(0, size_fold) as i32;
		NewContent {
			site_id: 0,
			inner_path: String::new(),
			size: 0,
			size_files,
			size_files_optional,
			modified: content.modified as i32,
		}
	}
}

#[derive(Queryable, Deserialize, Debug)]
pub struct Site {
	pub site_id: i32,
	pub address: String,
}

#[derive(Insertable)]
#[table_name = "site"]
pub struct NewSite<'a> {
	pub address: &'a str,
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name = "file_optional"]
pub struct FileOptional {
	pub file_id: i32,
	pub site_id: i32,
	pub inner_path: String,
	pub hash_id: i32,
	pub size: i32,
	pub peer: i32,
	pub uploaded: i32,
	pub is_downloaded: i32,
	pub is_pinned: i32,
	pub time_added: i32,
	pub time_downloaded: i32,
	pub time_accessed: i32,
}

#[derive(Insertable, Debug, Default)]
#[table_name = "file_optional"]
pub struct NewFileOptional {
	pub site_id: i32,
	pub inner_path: String,
	pub hash_id: i32,
	pub size: i32,
	pub peer: i32,
	pub uploaded: i32,
	pub is_downloaded: i32,
	pub is_pinned: i32,
	pub time_added: i32,
	pub time_downloaded: i32,
	pub time_accessed: i32,
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name = "json"]
pub struct FileJSON {
	pub json_id: i32,
	pub site: String,
	pub directory: String,
	pub file_name: String,
}
