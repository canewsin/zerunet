pub mod address;
mod site_info;
pub mod site_manager;
pub mod site_storage;

use crate::error::Error;
use crate::peer::Peer;
use crate::server::websocket::ZeruWebsocket;
use actix;
use actix::prelude::*;
use address::Address;
use futures::executor::block_on;
use log::*;
use serde_derive::{Deserialize, Serialize};
use site_info::{SiteInfo, SiteSettings};
use std::collections::HashMap;
use crate::server::websocket::ZeruWebsocket;

pub struct Site {
	address: Address,
	peers: HashMap<String, Addr<Peer>>,
	settings: SiteSettings,
	listeners: Vec<Addr<ZeruWebsocket>>,
}

impl Site {
	pub fn new(address: Address) -> Site {
		Site {
			address,
			peers: HashMap::new(),
			settings: SiteSettings::default(),
			listeners: Vec::new(),
		}
	}
	pub fn load_settings() {}
	pub fn save_settings() {}
	pub fn is_serving() {}
	pub fn get_settings_cache() {}
	pub fn get_size_limit() {}
	pub fn get_next_size_limit() {}
	pub fn download_content() {}
	pub fn get_reachable_bad_files() {}
	pub fn retry_bad_files() {}
	pub fn check_bad_files() {}
	pub fn download() {}
	pub fn pooled_download_content() {}
	pub fn pooled_download_file() {}
	pub fn updater() {}
	pub fn check_modifications() {}
	pub fn update() {}
	pub fn redownload_contents() {}
	pub fn publisher() {}
	pub fn publish() {}
	pub fn clone() {}
	pub fn pooled_need_file() {}
	pub fn is_file_download_allowed() {}
	pub fn need_file_info() {
		// TODO: get info for file from content.json
	}
	// Check and download if file not exist
	pub fn need_file(&mut self, inner_path: String) -> bool {
		if false {
			// get task if task exists
			return false;
		} else if true {
			// check if file exists
			return true;
		} else if !self.settings.serving {
			// Site is not serving
			return false;
		} else {
			// create task and wait for completion if blocking
			return true;
		}
	}
	pub fn add_peer() {}
	pub fn announce() {}
	pub fn need_connections() {}
	pub fn get_connectable_peers() {}
	pub fn get_recent_peers() {}
	pub fn get_connected_peers() {}
	pub fn cleanup_peers() {}
	pub fn send_my_hashfield() {}
	pub fn update_hashfield() {}
	pub fn is_downloadable() {}
	pub fn delete() {}
	pub fn add_event_listener() {}
	pub fn update_websocket() {}
	pub fn message_websocket() {}
	pub fn file_started() {}
	pub fn file_done() {}
	pub fn file_failed() {}
	pub fn file_forgot() {}
}

impl Actor for Site {
	type Context = Context<Self>;
}

pub struct SiteInfoRequest {}

impl Message for SiteInfoRequest {
	type Result = Result<SiteInfo, ()>;
}

impl Handler<SiteInfoRequest> for Site {
	type Result = Result<SiteInfo, ()>;

	fn handle(&mut self, msg: SiteInfoRequest, ctx: &mut Context<Self>) -> Self::Result {
		// TODO: replace default values
		Ok(SiteInfo {
			tasks: 0,
			size_limit: 10,
			address: self.address.to_string(),
			next_size_limit: 10,
			auth_address: String::from("test"),
			auth_key_sha512: String::from("test"),
			peers: self.peers.len() + 1, // TODO: only add 1 if hosting zite
			auth_key: String::from("test"),
			settings: self.settings.clone(),
			bad_files: 0,
			workers: 0,
			content: Default::default(),
			started_task_num: 0,
			content_updated: 0f64,
		})
	}
}

pub struct AddPeer {
	peer_id: String,
	peer_addr: Addr<Peer>,
}

impl Message for AddPeer {
	type Result = Result<(), ()>;
}

impl Handler<AddPeer> for Site {
	type Result = Result<(), ()>;

	fn handle(&mut self, msg: AddPeer, ctx: &mut Context<Self>) -> Self::Result {
		let prev = self.peers.insert(msg.peer_id.clone(), msg.peer_addr);
		if prev.is_none() {
			trace!(
				"Added {} as peer for {}",
				msg.peer_id,
				self.address.get_address_short()
			);
		}
		Ok(())
	}
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FileGetRequest {
	#[serde(default)]
	pub inner_path: String,
	#[serde(default)]
	pub required: bool,
	#[serde(default)]
	pub format: String,
	#[serde(default)]
	pub timeout: f64,
}

impl Message for FileGetRequest {
	type Result = Result<bool, Error>;
}

impl Handler<FileGetRequest> for Site {
	type Result = Result<bool, Error>;

	fn handle(&mut self, msg: FileGetRequest, _ctx: &mut Context<Self>) -> Self::Result {
		if self.peers.len() == 0 {
			trace!("No peers for {}", self.address.to_string());
			return Ok(false);
		}
		for (key, peer) in self.peers.iter() {
			if block_on(peer.send(msg.clone())).is_ok() {
				return Ok(true);
			}
		}
		return Ok(false);
	}
}
