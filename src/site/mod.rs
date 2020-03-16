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
use zerucontent::Content;
use std::io::Write;
use std::path::PathBuf;

pub struct Site {
	address: Address,
	peers: HashMap<String, Addr<Peer>>,
	settings: SiteSettings,
	content: Option<Content>,
	// TODO: queued files should be more than just a string,
	// they should have a priority, try-count, last-try timestamp,
	// 
	queued_files: Vec<String>,
	data_path: PathBuf,
	listeners: Vec<Addr<ZeruWebsocket>>,
}

impl Site {
	pub fn new(listeners: Vec<Addr<ZeruWebsocket>>, address: Address, data_path: PathBuf) -> Site {
		let mut settings = SiteSettings::default();
		settings.serving = true;
		Site {
			address,
			peers: HashMap::new(),
			settings,
			content: None,
			queued_files: Vec::new(),
			listeners,
			data_path,
		}
	}
	pub fn load_settings() {}
	pub fn save_settings() {}
	pub fn is_serving() {}
	pub fn get_settings_cache() {}
	pub fn get_size_limit() {}
	pub fn get_next_size_limit() {}
	// Download content files
	pub fn download_content(&mut self, inner_path: &str) -> Result<(), Error> {
		let buf = self.download_file(inner_path)?;
		let content = match Content::from_buf(buf) {
			Ok(c) => c, 
			Err(_) => return Err(Error::MissingError),
		};
		// TODO: verify content
		self.content = Some(content);
		Ok(())
	}
	pub fn get_reachable_bad_files() {}
	pub fn retry_bad_files() {}
	pub fn check_bad_files() {}
	// Initial download of site
	pub fn download_site(&mut self) -> Result<(), Error> {
		if self.content.is_none() {
			self.download_content("content.json")?;
		}
		// TODO: how to best avoid this clone?
		// I'd say instead of "needing" files here, adding them to a queue and
		// downloading them later instead would solve the problem adequately.
		let content = self.content.as_mut().unwrap().clone();
		if !content.verify(self.address.to_string()) {
			error!("Content signature {:?} is not valid for address {}!", content.signs, self.address.to_string());
			return Err(Error::MissingError)
		}
		for (key, value) in content.files.iter() {
			self.need_file(key); // TODO: handle result
		}
		Ok(())
	}
	// Download file 
	pub fn download_file(&mut self, inner_path: &str) -> Result<serde_bytes::ByteBuf, Error> {
		if self.peers.len() == 0 {
			trace!("No peers for {}", self.address.to_string());
			return Err(Error::MissingError);
		}
		// TODO: Do some smart peer management here instead of this ... whatever it is
		for (key, peer) in self.peers.iter() {
			let req = crate::peer::FileGetRequest {
				inner_path: inner_path.to_string(),
				site_address: self.address.clone(),
			};
			if let Ok(Ok(buf)) = block_on(peer.send(req)) {
				return Ok(buf)
			}
		}
		return Err(Error::MissingError);
	}
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
	pub fn need_file(&mut self, inner_path: &str) -> Result<bool, Error> {
		// TODO: move site download to appropriate place
		if self.content.is_none() {
			self.download_site()?;
		}
		let file_content = match self.content.as_ref().unwrap().get_file(inner_path) {
			Some(f) => f,
			None => return Err(Error::MissingError),
		};
		// TODO: find file in content and prioritize appropriately
		self.queued_files.push(String::from(inner_path));
		// TODO: stop here, let queued files be downloaded by routine
		let buf = self.download_file(inner_path)?;
		let mut path = self.data_path.clone();
		path.push(&self.address.to_string());
		path.push(inner_path);
		// TODO: check if we need to take the parent here
		trace!("{:?}", std::fs::create_dir_all(path.parent().unwrap()));
		let mut file = match std::fs::File::create(&path) {
			Ok(f) => f,
			Err(err) => {
				error!("Error creating '{:?}': {:?}", &path, err);
				return Err(Error::MissingError);
			}
		};
		if buf.len() != file_content.size {
			error!("Wrong filesize!");
			return Err(Error::MissingError);
		}
		let mut hasher = sha2::Sha512::default();
		use sha2::Digest;
		hasher.input(&buf);
		let mut hash_result = hex::encode(hasher.result());
		hash_result.truncate(64);
		if hash_result != file_content.sha512 {
			error!("Wrong filehash: {} != {}", &hash_result, &file_content.sha512);
			return Err(Error::MissingError);
		}
		file.write_all(&buf)?;
		return Ok(true);
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
		if self.content.is_none() {
			self.download_content("content.json");
		}
		Ok(SiteInfo {
			tasks: self.queued_files.len(),
			size_limit: 10,
			address: self.address.to_string(),
			address_short: self.address.get_address_short(),
			next_size_limit: 10,
			auth_address: String::from("test"),
			auth_key_sha512: String::from("test"),
			peers: self.peers.len() + 1, // TODO: only add 1 if hosting zite
			auth_key: String::from("test"),
			settings: self.settings.clone(),
			bad_files: 0,
			workers: 0,
			content: site_info::SiteContentSummary::from_content(&self.content.as_ref().unwrap()),
			started_task_num: 0,
			content_updated: 0f64,
		})
	}
}

/// Message struct used to add a peer to a site
/// ```
/// let site = Site::new(Vec::new(), Address::from_str("Demo"), "data_path");
/// let peer = Peer::new(PeerAddress::IPV4("192.168.1.1:5432"));
/// let msg = AddPeer{
/// 	peer_id: "peerID",
/// 	peer_addr: peer.addr(),
/// };
/// site.send(msg);
/// ```
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

/// Message struct used to request a file from a site
/// ```
/// match result {
/// 	Ok(true) => println!("File has been downloaded."),
/// 	Ok(false) => println!("File has been queued for download."),
/// 	Err(_) => println!("An error occured!"),
/// }
/// ```
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
		self.need_file(&msg.inner_path)
	}
}

/// Message struct used to add a websocket actor as
/// listener to a site
/// ```
/// match result {
/// 	Ok(true) => println!("Listener added."),
/// 	Ok(false) => println!("Listener was already added."),
/// 	Err(_) => println!("An error occured while adding listener!"),
/// }
pub struct ChannelJoinRequest {
	pub ws_addr: Addr<ZeruWebsocket>,
}

impl Message for ChannelJoinRequest {
	type Result = Result<bool, Error>;
}

impl Handler<ChannelJoinRequest> for Site {
	type Result = Result<bool, Error>;

	fn handle(&mut self, msg: ChannelJoinRequest, _ctx: &mut Context<Self>) -> Self::Result {
		let ws_addr = msg.ws_addr.clone();
		if !self.listeners.contains(&ws_addr) {
			self.listeners.push(ws_addr);
			return Ok(true)
		} else {
			return Ok(false)
		}
	}
}
