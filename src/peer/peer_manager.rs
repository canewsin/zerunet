use crate::error::Error;
use actix::{prelude::*, Actor, Addr};
use log::*;
use std::collections::HashMap;

pub struct PeerID(String);

pub struct PeerManager {
	peers: HashMap<String, Peer>,
}

impl PeerManager {
	pub fn new() -> PeerManager {
		PeerManager {
			peers: HashMap::new(),
		}
	}
	pub fn get(&mut self, peer_id: String) -> Result<Addr<Peer>, Error> {
		if let Some(addr) = self.peers.get(&peer_id) {
			Ok(addr.clone())
		} else {
			info!("Spinning up actor for peer {}", &peer_id);
			let peer = Peer::new();
			let addr = peer.start();
			self.peers.insert(peer_id, addr);
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

impl Handler<PeerLookup> for SiteManager {
	type Result = Result<Addr<Peer>, Error>;

	fn handle(&mut self, msg: PeerLookup, _ctx: &mut Context<Self>) -> Self::Result {
		match msg {
			PeerLookup::ID(peer_id) => self.get(peer_id),
		}
	}
}
