use super::Announcer;
use crate::error::Error;
use crate::peer::Peer;
use crate::site::address::Address;
use crate::util::is_default;
use actix::prelude::*;
use futures::executor::block_on;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::time::SystemTime;
use zeronet_protocol::templates;
use zeronet_protocol::Address as PeerAddress;

pub struct Announce {
	pub req: templates::Announce,
}

impl Message for Announce {
	type Result = Result<templates::AnnounceResponse, Error>;
}

pub struct ZeroAnnouncer {
	peer: Addr<Peer>,
}

impl ZeroAnnouncer {
	pub fn new(tracker: super::Tracker) -> ZeroAnnouncer {
		let peer =
			Peer::new(PeerAddress::parse(format!("{}:{}", tracker.address, tracker.port)).unwrap());
		let (tx, rx) = std::sync::mpsc::channel();
		std::thread::spawn(move || {
			actix::System::run(move || {
				let addr = peer.start();
				tx.send(addr);
			});
		});
		let addr = rx.recv().unwrap();
		ZeroAnnouncer { peer: addr }
	}
}

fn site_announce(addr: &Address) -> Announce {
	let req = templates::Announce {
		port: 11692,
		add: vec!["ipv4".to_string()],
		need_types: vec!["ipv4".to_string(), "ipv6".to_string()],
		need_num: 20,
		hashes: vec![serde_bytes::ByteBuf::from(addr.get_address_hash())],
		onions: vec![],
		onion_signs: vec![],
		onion_sign_this: String::new(),
		delete: false,
	};
	Announce { req }
}
fn full_announce() -> Announce {
	let req = templates::Announce {
		port: 0,
		add: vec![],
		need_types: vec![],
		need_num: 20,
		hashes: vec![],
		onions: vec![],
		onion_signs: vec![],
		onion_sign_this: String::new(),
		delete: true,
	};
	Announce { req }
}

impl Announcer for ZeroAnnouncer {
	fn announce(&mut self, address: &Address) -> Result<(), ()> {
		let start_time = SystemTime::now();

		// TODO:
		/***
		 * - need_types should be ip4 + supported_ip_types
		 * - if dark mode send separate announce per site hash + onion
		 *   else fill hashes + onions
		 * - port is fileserver_port
		 * - add_types = openedServiceTypes()
		 * - delete = true if full announce
			**/

		let request = site_announce(address);
		// let request = Announce::full();
		// let mut request = HashMap::new();
		// let hash = serde_bytes::ByteBuf::from(address.get_address_hash());
		// request.insert("hashes", json!(hash));

		// let res = block_on(conn.request("announce", json!(request)));

		trace!("Sending AnnounceRequest...");
		let res = block_on(self.peer.send(request));
		trace!("Received announce response: {:?}", res);
		Err(())
	}
}
