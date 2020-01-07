use crate::util::is_default;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
	cmd: MessageType,
	#[serde(skip_serializing_if = "is_default")]
	to: isize,
	result: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageType {
	Response,
	Error,
	Ping,
}

impl Message {
	pub fn new(id: isize, body: serde_json::Value) -> Message {
		Message {
			cmd: MessageType::Response,
			to: id,
			result: body,
		}
	}
}
