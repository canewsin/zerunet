use super::Site;

pub trait SiteStorage {
	fn get_db_file() {}
	fn open_db() {}
	fn close_db() {}
	fn get_db_schema() {}
	fn load_db() {}
	fn get_db() {}
	fn update_db_file() {}
	fn get_db_files() {}
	fn rebuild_db() {}
	fn query() {}
	fn ensure_dir() {}
	fn open() {}
	fn read() {}
	fn write_thread() {}
	fn write() {}
	fn delete() {}
	fn delete_dir() {}
	fn rename() {}
	fn walk() {}
	fn list() {}
	fn on_updated() {}
	fn load_json() {}
	fn write_json() {}
	fn get_size() {}
	fn is_file() {}
	fn is_exists() {}
	fn is_dir() {}
	fn get_path() {}
	fn get_inner_path() {}
	fn verify_files() {}
	fn update_bad_files() {}
	fn delete_files() {}
}

impl SiteStorage for Site {

}