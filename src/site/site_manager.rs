use super::{
	address::Address,
	Site,
};
use crate::error::Error;
use actix::{
	prelude::*,
	Actor, Addr,
};
use std::collections::HashMap;
use log::*;

pub struct SiteManager {
	sites: HashMap<Address, Addr<Site>>,
	nonce: HashMap<String, Addr<Site>>,
}

impl SiteManager {
	pub fn new() -> SiteManager {
		SiteManager {
			sites: HashMap::new(),
			nonce: HashMap::new(),
		}
	}
	pub fn get(&mut self, address: Address) -> Result<Addr<Site>, Error> {
		if let Some(addr) = self.sites.get(&address) {
			Ok(addr.clone())
		} else {
			info!("Spinning up actor for zero://{}", address.get_address_short());
			let site = Site::new();
			let addr = site.start();
			self.sites.insert(address, addr.clone());
			Ok(addr)
		}
	}
	pub fn get_by_key(&mut self, key: String) -> Result<Addr<Site>, Error> {
		if let Some(addr) = self.nonce.get(&key) {
			return Ok(addr.clone())
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
	type Result = Result<Addr<Site>, Error>;
}

impl Handler<Lookup> for SiteManager {
	type Result = Result<Addr<Site>, Error>;

	fn handle(&mut self, msg: Lookup, _ctx: &mut Context<Self>) -> Self::Result {
		match msg {
			Lookup::Address(address) => self.get(address),
			Lookup::Key(s) => self.get_by_key(s),
		}
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
impl Handler<AddWrapperKey> for SiteManager{
	type Result = Result<(), Error>;

	fn handle(&mut self, msg: AddWrapperKey, _ctx: &mut Context<Self>) -> Self::Result {
		let addr = self.get(msg.address.clone())?;
		self.nonce.insert(msg.wrapper_key.clone(), addr);
		info!("Added wrapper key {} for {}", msg.wrapper_key, msg.address.get_address_short());
		trace!("{:?}", self.sites);
		trace!("{:?}", self.nonce);
		Ok(())
	}
}