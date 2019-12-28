mod connections;

use crate::site::address::Address;
use ipnetwork::IpNetwork;
use chrono::{DateTime, Utc, Duration};
use actix::{prelude::*, Actor};

pub struct Peer {
	// connection: T,
	ip: IpNetwork,
	port: usize,
	site: Address,
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

impl Actor for Peer {
	type Context = Context<Self>;
}

pub struct PeerMessage {}

impl Message for PeerMessage {
	type Result = Result<(), ()>;
}
