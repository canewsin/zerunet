use derive_more::Display;

#[derive(Debug, Display)]
pub enum Error {
	ErrorBindingSocket,
	ErrorDecodingMessagePack(String),
	ErrorEncodingMessagePack(String),
	ErrorFindingIP,
	IncompatibleService,
	SenderSamePeerID,
	UnknownDiscoveryCommand,
}

impl From<std::io::Error> for Error {
	fn from(_: std::io::Error) -> Error {
		Error::ErrorBindingSocket
	}
}

impl From<rmp_serde::decode::Error> for Error {
	fn from(err: rmp_serde::decode::Error) -> Error {
		Error::ErrorDecodingMessagePack(err.to_string())
	}
}

impl From<rmp_serde::encode::Error> for Error {
	fn from(err: rmp_serde::encode::Error) -> Error {
		Error::ErrorEncodingMessagePack(err.to_string())
	}
}
