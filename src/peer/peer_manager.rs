use super::Peer;
use crate::error::Error;
use crate::site::site_manager::SiteManager;
use actix::{prelude::*, Actor, Addr};
use log::*;
use std::collections::HashMap;
use std::sync::mpsc::{channel, RecvError};
use zeronet_protocol::Address as PeerAddress;

// start_peer_manager starts the peer manager actor in a new system thread
pub fn start_peer_manager(
	site_manager_addr: Addr<SiteManager>,
) -> Result<Addr<PeerManager>, RecvError> {
	info!("Starting peer manager");

	let (sender, receiver) = channel();
	std::thread::spawn(move || {
		let peer_manager = PeerManager::new(site_manager_addr);
		let peer_manager_system = System::new("Peer manager");
		let peer_manager_addr = peer_manager.start();
		if sender.send(peer_manager_addr).is_err() {
			error!("Error sending peer manager address to main thread");
		}

		if peer_manager_system.run().is_err() {
			error!("Peer Manager Actix System encountered an error");
		}
	});
	receiver.recv()
}

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
	fn add(&mut self, peer_id: String, address: PeerAddress) -> Result<Addr<Peer>, Error> {
		if let Some(addr) = self.peers.get(&peer_id) {
			trace!("Peer {} was already known", &peer_id);
			// TODO: update the address
			Ok(addr.clone())
		} else {
			info!(
				"Spinning up actor for peer {} with address {}",
				&peer_id, address
			);
			let peer = Peer::new(address);
			let addr = peer.start();
			self.peers.insert(peer_id, addr.clone());
			Ok(addr)
		}
	}
	// Retrieves the peer's address, spinning up a new actor if the peer does not exist yet
	fn get(&mut self, peer_id: String) -> Result<Addr<Peer>, Error> {
		if let Some(addr) = self.peers.get(&peer_id) {
			Ok(addr.clone())
		} else {
			error!("Peer not found");
			Err(Error::MissingError)
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
	pub address: PeerAddress,
	pub peer_id: String,
	pub sites: Vec<Vec<u8>>,
}

impl Message for UpdatePeer {
	type Result = Result<(), Error>;
}

impl Handler<UpdatePeer> for PeerManager {
	type Result = Result<(), Error>;

	fn handle(&mut self, msg: UpdatePeer, _ctx: &mut Context<Self>) -> Self::Result {
		let addr = self.add(msg.peer_id.clone(), msg.address)?;
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
