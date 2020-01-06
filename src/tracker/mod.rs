use serde::{Serialize, Deserialize};

pub trait Tracker {
    fn announce();
    fn announcePex();
    fn getAddressParts();
    fn getAnnouncingTrackers();
    fn getOpenedServiceTypes();
    fn getSupportedTrackers();
    fn getTrackerHandler();
    fn getTrackers();
    fn updateWebsocket();
}

pub struct TrackerManager {
  // trackers: Vec<Tracker>,
}

pub struct SiteStats {

}

pub struct SiteAnnouncer {
  // site_address: Addr,
  stats: SiteStats,
  fileserver_port: usize,
  peer_id: String,
  last_tracker_id: usize,
  time_last_announce: usize,
}

impl SiteAnnouncer {
  pub fn getTrackers(&mut self) {}
  pub fn getSupportedTrackers(&mut self) {}
  pub fn getAnnouncingTrackers(&self) {}
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