mod connections;
pub mod peer_manager;

use crate::site::address::Address;
use ipnetwork::IpNetwork;
use chrono::{DateTime, Utc, Duration};
use actix::{prelude::*, Actor};

pub struct Peer {
	// connection: T,
	reputation: isize,
	time_found: DateTime<Utc>,
	time_added: DateTime<Utc>,
	time_response: DateTime<Utc>,
	last_content_json_update: DateTime<Utc>,
	download_bytes: usize,
	download_time: Duration,
	bad_files: usize,
	errors: usize,
}

impl Peer {
	pub fn new() -> Peer {
		Peer {
			reputation: 0,
			time_found: Utc::now(),
			time_added: Utc::now(),
			time_response: Utc::now(),
			last_content_json_update: Utc::now(),
			download_bytes: 0,
			download_time: Duration::seconds(0),
			bad_files: 0,
			errors: 0,
		}
	}
}

impl Actor for Peer {
	type Context = Context<Self>;
}

pub struct PeerMessage {}

impl Message for PeerMessage {
	type Result = Result<(), ()>;
}
