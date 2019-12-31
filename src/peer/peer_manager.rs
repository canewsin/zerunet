use super::Peer;
use crate::error::Error;
use crate::site::site_manager::SiteManager;
use actix::{prelude::*, Actor, Addr};
use log::*;
use std::collections::HashMap;

pub struct PeerID(String);

pub struct PeerManager {
	site_manager: Addr<SiteManager>,
	peers: HashMap<String, Addr<Peer>>,
}

impl PeerManager {
	pub fn new(site_manager: Addr<SiteManager>) -> PeerManager {
		PeerManager {
			site_manager,
			peers: HashMap::new(),
		}
	}
	// Retrieves the peer's address, spinning up a new actor if the peer does not exist yet
	fn get(&mut self, peer_id: String) -> Result<Addr<Peer>, Error> {
		if let Some(addr) = self.peers.get(&peer_id) {
			Ok(addr.clone())
		} else {
			info!("Spinning up actor for peer {}", &peer_id);
			let peer = Peer::new();
			let addr = peer.start();
			self.peers.insert(peer_id, addr.clone());
			Ok(addr)
		}
	}
}

impl Actor for PeerManager {
	type Context = Context<Self>;
}

#[derive(Debug)]
pub enum PeerLookup {
	ID(String),
}

impl Message for PeerLookup {
	type Result = Result<Addr<Peer>, Error>;
}

impl Handler<PeerLookup> for PeerManager {
	type Result = Result<Addr<Peer>, Error>;

	fn handle(&mut self, msg: PeerLookup, _ctx: &mut Context<Self>) -> Self::Result {
		match msg {
			PeerLookup::ID(peer_id) => self.get(peer_id),
		}
	}
}

pub struct UpdatePeer {
	pub peer_id: String,
	pub sites: Vec<Vec<u8>>,
}

impl Message for UpdatePeer {
	type Result = Result<(), Error>;
}

impl Handler<UpdatePeer> for PeerManager {
	type Result = Result<(), Error>;

	fn handle(&mut self, msg: UpdatePeer, _ctx: &mut Context<Self>) -> Self::Result {
		let addr = self.get(msg.peer_id.clone())?;
		self
			.site_manager
			.do_send(crate::site::site_manager::AddPeer {
				peer_id: msg.peer_id,
				peer_addr: addr,
				sites: msg.sites,
			});
		Ok(())
	}
}
