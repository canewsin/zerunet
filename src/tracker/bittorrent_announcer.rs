use super::Announcer;
use crate::site::address::Address;
use log::*;

pub struct BittorrentAnnouncer {}

impl Announcer for BittorrentAnnouncer {
	fn announce(&mut self, address: &Address) -> Result<(), ()> {
		warn!("Bittorrent announcer is not implemented");

		Ok(())
	}
}
