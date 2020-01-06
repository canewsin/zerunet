use super::{address::Address, site_info::SiteInfo, Site};
use crate::error::Error;
use crate::peer::Peer;
use actix::{prelude::*, Actor, Addr};
use chrono::{DateTime, Utc};
use log::*;
use std::collections::HashMap;

use futures::future::join_all;
use futures::future::{FutureExt, TryFutureExt};
use std::sync::mpsc::{channel, RecvError};

pub fn start_site_manager() -> Result<Addr<SiteManager>, RecvError> {
	info!("Starting site manager.");
	
	let (sender, receiver) = channel();
	std::thread::spawn(move || {
		let site_manager = SiteManager::new();
		let site_manager_system = System::new("Site manager");
		let site_manager_addr = site_manager.start();
		if sender.send(site_manager_addr).is_err() {
			error!("Error sending site manager address to main thread");
		}

		if site_manager_system.run().is_err() {
			error!("Site Manager Actix System encountered an error");
		};
	});
	receiver.recv()
}

pub struct SiteManager {
	sites: HashMap<Address, Addr<Site>>,
	nonce: HashMap<String, Address>,
	updated_at: DateTime<Utc>,
}

impl SiteManager {
	pub fn new() -> SiteManager {
		SiteManager {
			sites: HashMap::new(),
			nonce: HashMap::new(),
			updated_at: Utc::now(),
		}
	}
	pub fn get(&mut self, address: Address) -> Result<(Address, Addr<Site>), Error> {
		if let Some(addr) = self.sites.get(&address) {
			Ok((address, addr.clone()))
		} else {
			info!(
				"Spinning up actor for zero://{}",
				address.get_address_short()
			);
			let site = Site::new(address.clone());
			let addr = site.start();
			// TODO: Decide whether to spawn actors in syncArbiter
			// let addr = SyncArbiter::start(1, || Site::new());
			self.sites.insert(address.clone(), addr.clone());
			self.updated_at = Utc::now();
			Ok((address, addr))
		}
	}
	pub fn get_by_key(&mut self, key: String) -> Result<(Address, Addr<Site>), Error> {
		if let Some(address) = self.nonce.get(&key) {
			if let Some(addr) = self.sites.get(&address) {
				return Ok((address.clone(), addr.clone()));
			}
		}
		error!("No site found for key {}", key);
		Err(Error::MissingError)
	}
}

impl Actor for SiteManager {
	type Context = Context<Self>;
}

#[derive(Debug)]
pub enum Lookup {
	Address(Address),
	Key(String),
}

impl Message for Lookup {
	type Result = Result<(Address, Addr<Site>), Error>;
}

impl Handler<Lookup> for SiteManager {
	type Result = Result<(Address, Addr<Site>), Error>;

	fn handle(&mut self, msg: Lookup, _ctx: &mut Context<Self>) -> Self::Result {
		match msg {
			Lookup::Address(address) => self.get(address),
			Lookup::Key(s) => self.get_by_key(s),
		}
	}
}

pub struct SitesChangedRequest {}

impl Message for SitesChangedRequest {
	type Result = Result<DateTime<Utc>, Error>;
}

impl Handler<SitesChangedRequest> for SiteManager {
	type Result = Result<DateTime<Utc>, Error>;

	fn handle(&mut self, _msg: SitesChangedRequest, _ctx: &mut Context<Self>) -> Self::Result {
		Ok(self.updated_at)
	}
}

pub struct SiteListRequest {}

impl Message for SiteListRequest {
	type Result = Result<Vec<serde_bytes::ByteBuf>, Error>;
}

impl Handler<SiteListRequest> for SiteManager {
	type Result = Result<Vec<serde_bytes::ByteBuf>, Error>;

	fn handle(&mut self, _msg: SiteListRequest, _ctx: &mut Context<Self>) -> Self::Result {
		Ok(
			self
				.sites
				.iter()
				.map(|(key, _)| serde_bytes::ByteBuf::from(key.get_address_hash()))
				.collect(),
		)
	}
}

pub struct SiteInfoListRequest {}

impl Message for SiteInfoListRequest {
	type Result = Result<Vec<SiteInfo>, Error>;
}

impl Handler<SiteInfoListRequest> for SiteManager {
	type Result = ResponseActFuture<Self, Result<Vec<SiteInfo>, Error>>;

	fn handle(&mut self, _msg: SiteInfoListRequest, ctx: &mut Context<Self>) -> Self::Result {
		let requests: Vec<_> = self
			.sites
			.iter()
			.map(|(key, addr)| {
				addr.send(super::SiteInfoRequest {})
			})
			.collect();
		let request = join_all(requests)
			// .map_err(|_error| Error::MailboxError)
			.map(|r| {
				Ok(r.into_iter()
					.filter_map(|x| match x {
						Ok(Ok(a)) => Some(a),
						_ => None,
					}).collect())
			});
		let wrapped = actix::fut::wrap_future::<_, Self>(request);
		Box::new(wrapped)
	}
}

pub struct As {
	address: Address,
	command: crate::server::websocket::request::Command,
}

impl Message for As {
	type Result = Result<(), Error>;
}

impl Handler<As> for SiteManager {
	type Result = Result<(), Error>;

	fn handle(&mut self, msg: As, ctx: &mut Context<Self>) -> Self::Result {
		Ok(())
	}
}

pub struct AddWrapperKey {
	address: Address,
	wrapper_key: String,
}
impl AddWrapperKey {
	pub fn new(address: Address, wrapper_key: String) -> AddWrapperKey {
		AddWrapperKey {
			address: address,
			wrapper_key: wrapper_key,
		}
	}
}
impl Message for AddWrapperKey {
	type Result = Result<(), Error>;
}
impl Handler<AddWrapperKey> for SiteManager {
	type Result = Result<(), Error>;

	fn handle(&mut self, msg: AddWrapperKey, _ctx: &mut Context<Self>) -> Self::Result {
		let addr = self.get(msg.address.clone())?;
		self
			.nonce
			.insert(msg.wrapper_key.clone(), msg.address.clone());
		info!(
			"Added wrapper key {} for {}",
			msg.wrapper_key,
			msg.address.get_address_short()
		);
		Ok(())
	}
}

pub struct AddPeer {
	pub peer_id: String,
	pub peer_addr: Addr<Peer>,
	pub sites: Vec<Vec<u8>>,
}

impl Message for AddPeer {
	type Result = Result<(), ()>;
}

impl Handler<AddPeer> for SiteManager {
	type Result = Result<(), ()>;

	fn handle(&mut self, msg: AddPeer, _ctx: &mut Context<Self>) -> Self::Result {
		for (address, addr) in self.sites.iter_mut() {
			let hash = address.get_address_hash();
			for site in msg.sites.iter() {
				if hash == *site {
					addr.do_send(crate::site::AddPeer {
						peer_id: msg.peer_id.clone(),
						peer_addr: msg.peer_addr.clone(),
					})
				}
			}
		}
		Ok(())
	}
}
