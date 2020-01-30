pub mod connections;
pub mod peer_manager;

use crate::error::Error;
use crate::site::address::Address;
use crate::util::is_default;
use actix::{prelude::*, Actor};
use chrono::{DateTime, Duration, Utc};
use connections::{PeerAddress, Connection};
use ipnetwork::IpNetwork;
use log::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;

pub struct Peer {
	address: PeerAddress,
	connection: Option<Box<dyn Connection>>,
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
	pub fn request(&mut self) {}
	pub fn get_file(&mut self, address: &Address, inner_path: &String) -> Result<(), ()> {
		self.connect()?;
		if let Some(connection) = &mut self.connection {
			let mut params = HashMap::new();
			params.insert(String::from("site"), serde_json::json!(address.to_string()));
			params.insert(String::from("location"), serde_json::json!(0));
			params.insert(String::from("inner_path"), serde_json::json!(inner_path));
			// {'cmd': 'getHashfield', 'req_id': 1, 'params': {'site': '1CWkZv7fQAKxTVjZVrLZ8VHcrN6YGGcdky'}}
			let msg = PeerMessage {
				cmd: String::from("getFile"),
				to: 0,
				req_id: 1,
				params,
				zerunet: true,
				body: serde_bytes::ByteBuf::new(),
			};
			let response = connection.send(msg).unwrap();
			let mut path = std::path::PathBuf::from("/home/crolsi/Programs/zerunet/data/");
			path.push(&address.to_string());
			path.push(&inner_path);
			trace!("{:?}", std::fs::create_dir_all(path.parent().unwrap()));
			let mut file = match std::fs::File::create(&path) {
				Ok(f) => f,
				Err(err) => {
					error!("Error creating '{:?}': {:?}", &path, err);
					return Err(());
				}
			};
			file.write_all(&response.body);
		}

		warn!("Getting file from peer not fully implemented");
		Err(())
	}
	pub fn ping(&mut self) -> Result<(), ()> {
		self.connect()?;
		if let Some(connection) = &mut self.connection {
			let mut params = HashMap::new();
			let msg = PeerMessage {
				cmd: String::from("ping"),
				to: 0,
				req_id: 1,
				params,
				zerunet: true,
				body: serde_bytes::ByteBuf::new(),
			};
			let res = connection.send(msg);
			println!("{:?}", res);
		}

		error!("Pinging peer not implemented");
		Err(())
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

#[derive(Serialize, Deserialize, Debug)]
pub struct PeerMessage {
	cmd: String,
	#[serde(default, skip_serializing_if = "is_default")]
	to: usize,
	#[serde(default, skip_serializing_if = "is_default")]
	req_id: usize,
	#[serde(default, skip_serializing_if = "is_default")]
	params: HashMap<String, serde_json::Value>,
	#[serde(default)]
	zerunet: bool,
	#[serde(default, skip_serializing_if = "is_default")]
	body: serde_bytes::ByteBuf,
}

pub struct FileGetRequest {
	pub inner_path: String,
	pub site_address: Address,
}

impl Message for FileGetRequest {
	type Result = Result<bool, Error>;
}

impl Handler<FileGetRequest> for Peer {
	type Result = Result<bool, Error>;

	fn handle(&mut self, msg: FileGetRequest, _ctx: &mut Context<Self>) -> Self::Result {
		self.connect();
		self.get_file(&msg.site_address, &msg.inner_path);

		Ok(true)
	}
}
