use super::Announcer;
use crate::peer::connections::PeerAddress;
use crate::peer::Peer;
use crate::site::address::Address;
use crate::util::is_default;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::time::SystemTime;

pub struct ZeroAnnouncer {
	peer: Peer,
}

impl ZeroAnnouncer {
	pub fn new(tracker: super::Tracker) -> ZeroAnnouncer {
		ZeroAnnouncer {
			peer: Peer::new(PeerAddress::IPV4(tracker.address, tracker.port)),
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Announce {
	// #[serde(default, skip_serializing_if = "is_default")]
	hashes: Vec<serde_bytes::ByteBuf>,
	#[serde(default, skip_serializing_if = "is_default")]
	onions: Vec<String>,
	#[serde(default, skip_serializing_if = "is_default")]
	port: i64,
	#[serde(default, skip_serializing_if = "is_default")]
	need_types: Vec<String>,
	#[serde(default, skip_serializing_if = "is_default")]
	need_num: i64,
	#[serde(default, skip_serializing_if = "is_default")]
	add: Vec<String>,
	#[serde(default, skip_serializing_if = "is_default")]
	delete: bool,
}

impl Announce {
	fn site(addr: &Address) -> Announce {
		Announce {
			hashes: vec![serde_bytes::ByteBuf::from(addr.get_address_hash())],
			// hashes: vec![],
			onions: vec![],
			port: 11692,
			need_types: vec!["ip4", "ipv4", "ipv6"]
				.iter()
				.map(|s| s.to_string())
				.collect(),
			need_num: 20,
			add: vec!["ip4".to_string()],
			delete: false,
		}
	}
	fn full() -> Announce {
		Announce {
			hashes: vec![],
			onions: vec![],
			port: 0,
			need_types: vec![],
			need_num: 20,
			add: vec![],
			delete: true,
		}
	}
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

		let request = Announce::site(address);
		// let request = Announce::full();
		// let mut request = HashMap::new();
		// let hash = serde_bytes::ByteBuf::from(address.get_address_hash());
		// request.insert("hashes", json!(hash));

		let res = self.peer.request("announce", json!(request));
		trace!("Announce: {:?}", res);
		trace!("Announced in {:?}", start_time.elapsed().unwrap());
		Err(())
	}
}
