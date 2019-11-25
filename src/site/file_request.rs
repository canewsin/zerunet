use super::address::Address;
use std::path::{PathBuf, Path};

enum Priority {
  Content(usize),
  Site,
  Database,
  Required, // Files required by content.json
  Optional, // Optional files requested by ZeroFrame
  Default,
  Background, // Friend's files, preloading etc.
}

impl Priority {
  pub fn from_path(path: &Path) -> Result<Priority, ()> {
    if let Some(s) = path.to_str() {
      if s == "content.json" {
        return Ok(Priority::Content(0))
      }
    }
    if let Some(file_name) = match path.file_name() {
      None => return Err(()),
      Some(f) => f.to_str(),
    } {
      let priority = match file_name {
        "content.json" => Priority::Content(path.ancestors().count()),
        "index.html" => Priority::Site,
        "all.css" => Priority::Site,
        "all.js" => Priority::Site,
        "dbschema.json" => Priority::Database,
        _ => Priority::Default,
      };
      return Ok(priority)
    } else {
      return Err(())
    }
  }
}

enum Status {
  Waiting,
  Downloading,
  Paused,
  Failed,
}

struct FileRequest {
  address: Address,
  path: PathBuf,
  status: Status,
  priority: Priority,
}
