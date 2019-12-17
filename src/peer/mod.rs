use crate::site::address::Address;
use ipnetwork::IpNetwork;
use std::time::{Duration, SystemTime};

pub trait Connection {}

pub struct Peer<T: Connection> {
	connection: T,
	ip: IpNetwork,
	port: usize,
	site: Address,
	reputation: isize,
	time_found: SystemTime,
	time_added: SystemTime,
	time_response: SystemTime,
	last_content_json_update: SystemTime,
	download_bytes: usize,
	download_time: Duration,
	bad_files: usize,
	errors: usize,
}

pub struct PeerMessage {}