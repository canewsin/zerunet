mod content_db;
use content_db::ContentDb;
use std::path::{ Path, PathBuf };
use log::{error, trace, info};
use std::io::BufReader;
use std::fs::File;
use super::content::Content;

pub struct ContentManager {
  path: PathBuf,
  site: String,
  db: ContentDb,
}

impl ContentManager {
  // pub fn load_contents() {}
  // pub fn get_file_changes()
  pub fn load_content(&mut self, inner_path: PathBuf) {
    let content_path = self.path
      .join(Path::new(&self.site))
      .join(&inner_path);
    if !content_path.is_file() {
      error!("Path is not a file");
      return
    }
    let content_file = match File::open(content_path.as_path()) {
      Ok(f) => f,
      Err(_) => return,
    };
    let content: Content = match serde_json::from_reader(
      BufReader::new(content_file)) {
        Ok(c) => c,
        Err(_) => {
          error!("Could not read content.json");
          return
        }
      };
    // let old_content = self.db.get(inner_path);

  }
  // pub fn remove_content()
  // pub fn get_total_size()
  // pub fn list_modified()
  // pub fn list_contents()
  // pub fn is_archived()
  // pub fn is_downloaded()
  // pub fn is_modified()
  // pub fn get_file_info()
  // pub fn get_rules()
  // pub fn get_user_content_rules()
  // pub fn get_diffs()
  // pub fn hash_file()
  // pub fn is_valid_relative_path()
  // pub fn sanitize_path()
  // pub fn hash_files()
  // pub fn sign()
  // pub fn get_valid_signers()
  // pub fn get_signs_required()
  // pub fn verify_cert()
  // pub fn verify_content()
  // pub fn verify_content_include()
  // pub fn verify_file()
  // pub fn optional_delete()
  // pub fn optional_download()
  // pub fn optional_removed()
  // pub fn optional_renamed()
}
