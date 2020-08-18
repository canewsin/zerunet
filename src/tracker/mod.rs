use crate::environment::Environment;
use crate::site::address::Address;
use log::*;
use serde::{Deserialize, Serialize};

mod bittorrent_announcer;
pub mod zero_announcer;

use bittorrent_announcer::BittorrentAnnouncer;
use zero_announcer::ZeroAnnouncer;

// pub fn start_tracker_manager(
// 	env: &Environment,
// 	// site_manager_addr: Addr<TrackerManager>,
// ) -> Result<Addr<TrackerManager>, RecvError> {
// 	info!("Starting tracker manager");

// 	let (sender, receiver) = channel();
// 	std::thread::spawn(move || {
// 		let tracker_manager = TrackerManager::new();
// 		for tracker in &env.trackers {
// 			tracker_manager.add_tracker(&tracker);
// 		}
// 		let tracker_manager_system = System::new("Tracker manager");
// 		let tracker_manager_addr = tracker_manager.start();
// 		if sender.send(tracker_manager_addr).is_err() {
// 			error!("Error sending tracker manager address to main thread");
// 		}
// 		if tracker_manager_system.run().is_err() {
// 			error!("Tracker Manager Actix System encountered an error");
// 		}
// 	});
// 	receiver.recv()
// }

pub trait Announcer {
	fn announce(&mut self, site: &Address) -> Result<(), ()>;
}

// pub trait Tracker {
// 	fn announce();
// 	fn announce_pex();
// 	fn get_address_parts();
// 	fn get_announcing_trackers();
// 	fn get_opened_service_types();
// 	fn get_supported_trackers();
// 	fn get_tracker_handler();
// 	fn get_trackers();
// 	fn update_websocket();
// }

#[derive(Debug)]
enum Protocol {
	Zero,
	Http,
	Https,
	Udp,
}

pub struct Tracker {
	protocol: Protocol,
	address: String,
	port: usize,
}

impl Tracker {
	pub fn new(address: &str) -> Result<Tracker, ()> {
		let parts: Vec<&str> = address.split("://").collect();
		if parts.len() < 2 {
			return Err(());
		}

		let port = parts[1].split(":").last().unwrap().parse::<usize>();
		if port.is_err() {
			return Err(());
		}
		let port = port.unwrap();

		let address = parts[1].split(":").next().unwrap().to_string();

		let protocol = match parts[0] {
			"zero" => Protocol::Zero,
			"http" => Protocol::Http,
			"https" => Protocol::Https,
			"udp" => Protocol::Udp,
			_ => {
				error!("'{}' is not a recognized protocol", parts[0]);
				return Err(());
			}
		};

		let result = Tracker {
			protocol,
			address,
			port,
		};

		Ok(result)
	}
}

pub struct TrackerManager {
	trackers: Vec<Box<dyn Announcer>>,
	dark_mode: bool,
}

impl TrackerManager {
	pub fn new() -> TrackerManager {
		TrackerManager {
			trackers: vec![],
			dark_mode: false,
		}
	}
	pub fn add_tracker(&mut self, address: &str) -> Result<(), ()> {
		let tracker = Tracker::new(address)?;

		match tracker.protocol {
			Protocol::Zero => self.trackers.push(Box::new(ZeroAnnouncer::new(tracker))),
			Protocol::Http => self.trackers.push(Box::new(BittorrentAnnouncer {})),
			_ => {
				error!("Protocol '{:?}' is not supported", tracker.protocol);
				return Err(());
			}
		}

		trace!("Added {}", address);
		Ok(())
	}
	pub fn announce(&mut self, address: &Address) -> Result<(), ()> {
		for tracker in self.trackers.iter_mut() {
			tracker.announce(address)?;
		}
		Ok(())
	}
}

// pub struct SiteStats {}

// pub struct SiteAnnouncer {
//  site_address: Addr,
// 	stats: SiteStats,
// 	fileserver_port: usize,
// 	peer_id: String,
// 	last_tracker_id: usize,
// 	time_last_announce: usize,
// }

// impl SiteAnnouncer {
// 	pub fn get_trackers(&mut self) {}
// 	pub fn get_supported_trackers(&mut self) {}
// 	pub fn get_announcing_trackers(&self) {}
// }

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AnnouncerStats {
	pub status: String,
	pub num_request: usize,
	pub num_success: usize,
	pub num_error: usize,
	pub time_request: f64,
	pub time_last_error: f64,
	pub time_status: f64,
	pub last_error: String,
}
