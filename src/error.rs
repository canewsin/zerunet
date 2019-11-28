use actix_http::error::ResponseError;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum Error {
	FileNotFound,
	Deserialization,
}

impl From<std::io::Error> for Error {
	fn from(error: std::io::Error) -> Error {
		match error {
			_ => Error::FileNotFound,
		}
	}
}

impl From<serde_json::Error> for Error {
	fn from(error: serde_json::Error) -> Error {
		match error {
			_ => Error::Deserialization,
		}
	}
}

impl ResponseError for Error {}