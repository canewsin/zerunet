use crate::error::Error;
use actix_files::NamedFile;
use actix_web::{web::Data, HttpRequest, Result};
use log::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Info {
	nonce: String,
}

pub fn serve_file(
	req: &HttpRequest,
	_data: Data<crate::server::ZeroServer>,
) -> Result<NamedFile, Error> {
	let mut file_path = PathBuf::new();
	let address = req.match_info().query("address");
	let inner_path = req.match_info().query("inner_path");
	if address == "Test" {
		file_path.push(&Path::new("test/wrapper/public"));
	} else {
		file_path = PathBuf::from("../ZeroNet/data/");
		file_path.push(&Path::new(address));
	}
	file_path.push(&Path::new(inner_path));

	if file_path.is_dir() {
		file_path = file_path.join(PathBuf::from("index.html"));
	}

	trace!(
		"Serving file: zero://{}/{} as {:?}",
		req.match_info().query("address"),
		req.match_info().query("inner_path"),
		file_path
	);

	let file = NamedFile::open(file_path)?;
	Result::Ok(file)
}
