pub mod connections;
pub mod peer_manager;

use crate::error::Error;
use crate::site::address::Address as SiteAddress;
use crate::util::is_default;
use actix::{prelude::*, Actor};
use chrono::{DateTime, Duration, Utc};
use connections::{Connection, PeerAddress};
use ipnetwork::IpNetwork;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::io::Write;

pub struct Peer {
	address: PeerAddress,
	connection: Option<Box<dyn Connection<PeerMessage>>>,
	reputation: isize,
	time_found: DateTime<Utc>,
	time_added: DateTime<Utc>,
	time_response: DateTime<Utc>,
	last_content_json_update: DateTime<Utc>,
	download_bytes: usize,
	download_time: Duration,
	bad_files: usize,
	errors: usize,
}

impl Peer {
	pub fn new(address: PeerAddress) -> Peer {
		Peer {
			address,
			connection: None,
			reputation: 0,
			time_found: Utc::now(),
			time_added: Utc::now(),
			time_response: Utc::now(),
			last_content_json_update: Utc::now(),
			download_bytes: 0,
			download_time: Duration::seconds(0),
			bad_files: 0,
			errors: 0,
		}
	}
	pub fn connect(&mut self) -> Result<(), ()> {
		if self.connection.is_some() {
			return Ok(());
		}
		self.connection = Some(self.address.connect()?);
		Ok(())
	}
	pub fn request(
		&mut self,
		cmd: &str,
		params: serde_json::Value,
	) -> Result<serde_bytes::ByteBuf, ()> {
		self.connect()?;
		if let Some(connection) = &mut self.connection {
			let msg = PeerMessage {
				cmd: cmd.to_string(),
				to: None,
				req_id: Some(1),
				params,
				body: serde_bytes::ByteBuf::new(),
				peers: vec![],
			};
			let response = connection.request(msg);
			return match response {
				Err(err) => {
					error!("Invalid response: {:?}", err);
					Err(())
				}
				Ok(res) => Ok(res.body),
			};
		}

		Err(())
	}
	pub fn get_file(
		&mut self,
		address: &SiteAddress,
		inner_path: &String,
	) -> Result<serde_bytes::ByteBuf, ()> {
		warn!("Get file is not fully implemented");
		let mut params = HashMap::new();
		params.insert("site", json!(address.to_string()));
		params.insert("location", json!(0));
		params.insert("inner_path", json!(inner_path));
		// {'cmd': 'getHashfield', 'req_id': 1, 'params': {'site': '1CWkZv7fQAKxTVjZVrLZ8VHcrN6YGGcdky'}}
		return self.request("getFile", json!(params));
	}
	pub fn ping(&mut self) -> Result<(), ()> {
		let res = self.request("ping", serde_json::Value::Null)?;
		println!("{:?}", res);

		Ok(())
	}
	fn pex() {}
	fn list_modified() {}
	fn update_hashfield() {}
	fn find_hash_ids() {}
	fn send_my_hashfield() {}
	fn publish() {}
	fn remove() {}
	fn on_connection_error() {}
	fn on_worker_done() {}
}

impl Actor for Peer {
	type Context = Context<Self>;
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PeerCommand {
	StreamFile,
	GetFile,
	GetHashfield,
	Response,
	Handshake,
	Ping,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PeerMessage {
	cmd: String,
	#[serde(default, skip_serializing_if = "is_default")]
	req_id: Option<usize>,
	#[serde(default, skip_serializing_if = "is_default")]
	to: Option<usize>,
	#[serde(default, skip_serializing_if = "is_default")]
	params: serde_json::Value,
	#[serde(default, skip_serializing_if = "is_default")]
	body: serde_bytes::ByteBuf,
	#[serde(default, skip_serializing_if = "is_default")]
	peers: Vec<HashMap<String, serde_bytes::ByteBuf>>,
}

pub struct FileGetRequest {
	pub inner_path: String,
	pub site_address: SiteAddress,
}

impl Message for FileGetRequest {
	type Result = Result<serde_bytes::ByteBuf, Error>;
}

impl Handler<FileGetRequest> for Peer {
	type Result = Result<serde_bytes::ByteBuf, Error>;

	fn handle(&mut self, msg: FileGetRequest, _ctx: &mut Context<Self>) -> Self::Result {
		self.connect();
		if let Ok(buf) = self.get_file(&msg.site_address, &msg.inner_path) {
			Ok(buf)
		} else {
			Err(Error::MissingError)
		}
	}
}
