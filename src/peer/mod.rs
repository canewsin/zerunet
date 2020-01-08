mod connections;
pub mod peer_manager;

use crate::error::Error;
use crate::site::address::Address;
use crate::util::is_default;
use actix::{prelude::*, Actor};
use chrono::{DateTime, Duration, Utc};
use connections::Connection;
use ipnetwork::IpNetwork;
use log::*;
use serde::{Deserialize, Serialize};

pub struct Peer {
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
	pub fn new() -> Peer {
		Peer {
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
	pub fn connect(&mut self) {
		error!("Connecting to peer not implemented");
	}
	pub fn request(&mut self) {}
	pub fn get_file(&mut self) {
		error!("Getting file from peer not implemented");
	}
	pub fn ping() {}
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
	Response,
	Handshake,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PeerMessage {
	cmd: PeerCommand,
	#[serde(default, skip_serializing_if = "is_default")]
	to: String,
}

impl Message for PeerMessage {
	type Result = Result<(), ()>;
}

use crate::site::FileGetRequest;
impl Handler<FileGetRequest> for Peer {
	type Result = Result<bool, Error>;

	fn handle(&mut self, msg: FileGetRequest, _ctx: &mut Context<Self>) -> Self::Result {
		self.connect();
		self.get_file();

		Ok(true)
	}
}
