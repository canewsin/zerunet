
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::util::is_default;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Sender {
	pub service: String,
	pub peer_id: String,
	pub port: usize,
	pub broadcast_port: usize,
	pub rev: usize,
	#[serde(skip)]
	pub ip: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(default)]
pub struct Params {
	#[serde(skip_serializing_if = "is_default")]
	pub sites_changed: i64,
	#[serde(skip_serializing_if = "is_default")]
	pub sites: Vec<serde_bytes::ByteBuf>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct DiscoveryMessage {
	#[serde(skip_serializing_if = "is_default")]
	pub params: Params,
	pub sender: Sender,
	// TODO: cmd should be an enum, but mrp does not
	// serialize/deserialize those correctly
	pub cmd: String, 
}

use LocalDiscoveryCommand::*;
impl DiscoveryMessage {
	pub fn new(sender: Sender, cmd: LocalDiscoveryCommand) -> DiscoveryMessage {
		DiscoveryMessage {
			sender,
			cmd: cmd.to_string(),
			params: Params::default(),
		}
	}
	// Insert a parameter into the message
	pub fn add_param(&mut self, name: &str, value: serde_json::Value) {
		// self.params.insert(name.to_string(), value);
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum LocalDiscoveryCommand {
	DiscoverRequest,
	SiteListRequest,
	DiscoverResponse,
	SiteListResponse,
}

impl ToString for LocalDiscoveryCommand {
	fn to_string(&self) -> String {
		match self {
			DiscoverRequest => "discoverRequest",
			SiteListRequest => "siteListRequest",
			DiscoverResponse => "discoverResponse",
			SiteListResponse => "siteListResponse",
		}.to_string()
	}
}
