pub mod address;
mod site_info;
pub mod site_manager;

use crate::peer::Peer;
use actix;
use actix::prelude::*;
use address::Address;
use log::*;
use site_info::SiteInfo;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Site {
	address: Address,
	peers: HashMap<String, Addr<Peer>>,
}

impl Site {
	pub fn new(address: Address) -> Site {
		Site {
			address,
			peers: HashMap::new(),
		}
	}
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
		Ok(SiteInfo {
			tasks: 0,
			size_limit: 10,
			address: self.address.to_string(),
			next_size_limit: 10,
			auth_address: String::from("test"),
			auth_key_sha512: String::from("test"),
			peers: 1,
			auth_key: String::from("test"),
			settings: site_info::SiteSettings {
				peers: 0,
				serving: true,
				modified: 0f64,
				own: false,
				permissions: vec![String::from("ADMIN")],
				size: 0,
			},
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
