use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Error {}

impl From<serde_json::Error> for Error {
	fn from(_err: serde_json::Error) -> Error {
		Error {}
	}
}
