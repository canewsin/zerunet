use crate::error::Error;
use actix_files::NamedFile;
use actix_web::{web::Data, HttpRequest, Result};
use futures::executor::block_on;
use log::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct Info {
	nonce: String,
}

pub fn serve_file(
	req: &HttpRequest,
	data: Data<crate::server::ZeroServer>,
) -> Result<NamedFile, Error> {
	let mut file_path = PathBuf::new();
	let address = req.match_info().query("address");
	let mut inner_path = String::from(req.match_info().query("inner_path"));
	if address == "Test" {
		file_path.push(&Path::new("test/wrapper/public"));
	} else {
		file_path = data.data_path.clone();
		file_path.push(&Path::new(address));
	}
	file_path.push(&Path::new(&inner_path));

	// TODO: what if a file doesn't have an extension?
	if file_path.is_dir() || !inner_path.contains(".") {
		file_path = file_path.join(PathBuf::from("index.html"));
		// TODO: should we edit inner_path here? or just create a new one?
		inner_path = format!("{}/index.html", &inner_path)
			.trim_start_matches("/")
			.to_string();
	}

	trace!(
		"Serving file: zero://{}/{} as {:?}",
		&address,
		&inner_path,
		file_path
	);

	if !file_path.exists() {
		let lookup =
			crate::site::site_manager::Lookup::Address(crate::site::address::Address::from_str(address)?);
		let (_, addr) = block_on(data.site_manager.send(lookup))??;
		let msg = crate::site::FileGetRequest {
			inner_path: String::from(inner_path),
			format: String::new(),
			timeout: 0f64,
			required: true,
		};
		let res = block_on(addr.send(msg))??;
		if !res {
			return Result::Err(Error::MissingError);
		}
	}

	let file = NamedFile::open(file_path)?;
	Result::Ok(file)
}
