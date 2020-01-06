use serde::{Deserialize, Serialize};

pub trait Tracker {
	fn announce();
	fn announce_pex();
	fn get_address_parts();
	fn get_announcing_trackers();
	fn get_opened_service_types();
	fn get_supported_trackers();
	fn get_tracker_handler();
	fn get_trackers();
	fn update_websocket();
}

pub struct TrackerManager {
	// trackers: Vec<Tracker>,
}

pub struct SiteStats {}

pub struct SiteAnnouncer {
	// site_address: Addr,
	stats: SiteStats,
	fileserver_port: usize,
	peer_id: String,
	last_tracker_id: usize,
	time_last_announce: usize,
}

impl SiteAnnouncer {
	pub fn get_trackers(&mut self) {}
	pub fn get_supported_trackers(&mut self) {}
	pub fn get_announcing_trackers(&self) {}
}

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
