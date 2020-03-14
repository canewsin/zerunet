use actix_http::error::ResponseError;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum Error {
	FileNotFound,
	Deserialization(serde_json::Error),
	MissingError,
	ReqwestError,
	MsgPackEncoding,
	MsgPackDecoding(rmp_serde::decode::Error),
	MailboxError,
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
			_ => Error::Deserialization(error),
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
			_ => Error::MsgPackDecoding(error),
		}
	}
}

impl From<actix::MailboxError> for Error {
	fn from(error: actix::MailboxError) -> Error {
		match error {
			_ => Error::MailboxError,
		}
	}
}

impl ResponseError for Error {}
