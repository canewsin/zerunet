use serde::{Serialize, Deserialize};
use crate::util::is_default;
use crate::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Message {
	cmd: MessageType,
	#[serde(skip_serializing_if = "is_default")]
	to: isize,
	result: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub enum MessageType {
	Response,
	Error,
	Ping,
}

impl Message {
	pub fn respond<T: Serialize>(req: &super::request::Command, body: T) -> Result<Message, Error> {
		Ok(Message {
			cmd: MessageType::Response,
			to: req.id,
			result: serde_json::to_value(body)?,
		})
	}
}