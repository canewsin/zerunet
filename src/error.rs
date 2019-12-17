use actix_http::error::ResponseError;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum Error {
	FileNotFound,
	Deserialization,
	MissingError,
	ReqwestError,
	MsgPackEncoding,
	MsgPackDecoding,
}

impl From<reqwest::Error> for Error {
	fn from(error: reqwest::Error) -> Error {
		match error {
			_ => Error::ReqwestError,
		}
	}
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

impl From<rmp_serde::encode::Error> for Error {
	fn from(error: rmp_serde::encode::Error) -> Error {
		match error {
			_ => Error::MsgPackEncoding,
		}
	}
}

impl From<rmp_serde::decode::Error> for Error {
	fn from(error: rmp_serde::decode::Error) -> Error {
		match error {
			_ => Error::MsgPackDecoding,
		}
	}
}

impl ResponseError for Error {}
