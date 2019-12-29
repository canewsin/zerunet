mod error;
mod message;

use actix::{prelude::*, Actor, Context};
use log::*;
use std::net::UdpSocket;
use serde_json::json;

use crate::site::site_manager::SiteManager;
use crate::peer::peer_manager::PeerManager;

use error::Error;
use message::*;


pub struct BroadcastRequest{}

impl Message for BroadcastRequest {
	type Result = Result<(), Error>;
}

impl Actor for LocalDiscoveryServer {
	type Context = Context<Self>;
}

impl Message for DiscoveryMessage {
	type Result = Result<(), Error>;
}

impl Handler<DiscoveryMessage> for LocalDiscoveryServer {
	type Result = Result<(), Error>;

	fn handle(&mut self, msg: DiscoveryMessage, _ctx: &mut Context<Self>) -> Self::Result {
		self.handle_message(msg)
	}
}

impl Handler<BroadcastRequest> for LocalDiscoveryServer {
	type Result = Result<(), Error>;

	fn handle(&mut self, _msg: BroadcastRequest, _ctx: &mut Context<Self>) -> Self::Result {
		trace!("handling broadcast request");
		self.broadcast()
	}
}

const BROADCAST_PORT: usize = 1544;

pub fn start_local_discovery(site_manager: Addr<SiteManager>, peer_manager: Addr<PeerManager>) -> Result<(), Error> {
	let port = BROADCAST_PORT;
	let mut local_ips = Vec::new();
	for mut iface in pnet::datalink::interfaces() {
		iface.ips.iter_mut().for_each(|ip| local_ips.push(ip.ip().to_string()));
	}
	info!("Ips {:?}", &local_ips);
	let prob_ip = local_ips.iter().find(|ip| ip.starts_with("192.168.1"));
	if prob_ip.is_none() {
		error!("Could not find local ip!");
		return Err(Error::ErrorFindingIP);
	}
	let ip = prob_ip.unwrap();
	trace!("Listening on {}:{}", &ip, port);
	let socket = UdpSocket::bind(format!("{}:{}", &ip, port))?;

	let system = actix::System::new("Local discovery server");
	let lds = LocalDiscoveryServer::new(ip, site_manager, peer_manager)?;
	let discovery_addr = lds.start();

	discovery_addr.do_send(BroadcastRequest{});

	std::thread::spawn(move || {
		loop {
			match broadcast_listen(&socket) {
				Ok(msg) => discovery_addr.do_send(msg),
				Err(err) => error!("Malformed local discovery message {:?}", err),
			}
		}
	});

	system.run()?; // TODO: necessary?
	Ok(())
}

fn broadcast_listen(socket: &UdpSocket) -> Result<DiscoveryMessage, Error> {
	let mut buf = [0u8; 4800]; // = 32bytes * 100sites + 1600reserve
	let (amt, addr) = socket.recv_from(&mut buf).unwrap();
	let filled_buf = &mut buf[..amt];
	let vec = filled_buf.to_vec();
	let mut msg: DiscoveryMessage = rmp_serde::from_read_ref(&vec)?;
	msg.sender.ip = format!("{}", addr.ip());
	Ok(msg)
}

pub struct LocalDiscoveryServer {
	listen_port: usize,
	listen_ip: String,
	socket: UdpSocket,
	sender: Sender,
	site_manager: Addr<SiteManager>,
	peer_manager: Addr<PeerManager>,
}

impl LocalDiscoveryServer {
	pub fn new(ip: &str, site_manager: Addr<SiteManager>, peer_manager: Addr<PeerManager>) -> Result<LocalDiscoveryServer, Error> {
		let socket = UdpSocket::bind(format!("{}:{}", ip, BROADCAST_PORT+1))?;
		let vec: Vec<u8> = (0..12).map(|_| rand::random::<u8>()).collect();
		// TODO: Bittorrent style id "-UT3530-%s" % CryptHash.random(12, "base64")
		let peer_id = format!("-UT3530-{}", base64::encode(&vec));
		info!("Generated id: {}", peer_id);
		let sender = Sender {
			service: String::from("zeronet"),
			peer_id,
			ip: String::from(ip),
			port: 11692,
			broadcast_port: BROADCAST_PORT,
			rev: 4241,
		};
		Ok(LocalDiscoveryServer {
			listen_ip: String::from(ip),
			listen_port: BROADCAST_PORT,
			socket,
			sender,
			site_manager,
			peer_manager,
		})
	}
	fn send(&self, addr: &str, msg: DiscoveryMessage) -> Result<(), Error> {
		info!("Sending {} message to {}", msg.cmd, addr);
		let bytes = rmp_serde::to_vec_named(&msg)?;
		self.socket.send_to(&bytes, addr)?;
		Ok(())
	}
	pub fn broadcast(&self) -> Result<(), Error> {
		let msg = DiscoveryMessage::new(self.sender.clone(), LocalDiscoveryCommand::DiscoverRequest);
		trace!("Broadcasting {:?}", msg.cmd);
		let bytes = rmp_serde::to_vec_named(&msg)?;
		let socket = UdpSocket::bind(format!("{}:{}", &self.listen_ip, 0))?;
		socket.set_broadcast(true)?;
		let addr = format!("255.255.255.255:{}", self.listen_port);
		socket.send_to(&bytes, addr)?;
		Ok(())
	}
	fn handle_message(&self, msg: DiscoveryMessage) -> Result<(), Error> {
		if &msg.sender.service != "zeronet" {
			return Err(Error::IncompatibleService);
		}
		if &msg.sender.peer_id == &self.sender.peer_id {
			return Err(Error::SenderSamePeerID);
		}
		let response = match msg.cmd.as_str() {
			"discoverRequest" => self.handle_discovery_request(&msg),
			"discoverResponse" => self.handle_discovery_response(&msg),
			"siteListRequest" => self.handle_sitelist_request(&msg),
			"siteListResponse" => self.handle_sitelist_response(&msg),
			_ => return Err(Error::UnknownDiscoveryCommand),
		}?;

		let addr = format!("{}:{}", &msg.sender.ip, &msg.sender.broadcast_port);
		for resp in response {
			self.send(&addr, resp)?;
		}
		Ok(())
	}
	fn handle_discovery_request(&self, msg: &DiscoveryMessage) -> Result<Vec<DiscoveryMessage>, Error> {
		let sites_changed: Vec<String> = vec![];
		let mut resp = DiscoveryMessage::new(self.sender.clone(), LocalDiscoveryCommand::DiscoverResponse);
		let sites_changed = match self.site_manager.send(crate::site::site_manager::SitesChangedRequest{}).wait() {
			Ok(c) => c.unwrap(),
			Err(_) => return Err(Error::CouldNotGetSitesChanged),
		};
		trace!("Sites updated: {}", sites_changed);
		resp.params.sites_changed = sites_changed.timestamp();
		Ok(vec![resp])
	}
	fn handle_discovery_response(&self, msg: &DiscoveryMessage) -> Result<Vec<DiscoveryMessage>, Error> {
		let resp = DiscoveryMessage::new(self.sender.clone(), LocalDiscoveryCommand::SiteListRequest);
		Ok(vec![resp])
	}
	fn handle_sitelist_request(&self, msg: &DiscoveryMessage) -> Result<Vec<DiscoveryMessage>, Error> {
		let mut resp = DiscoveryMessage::new(self.sender.clone(), LocalDiscoveryCommand::SiteListResponse);
		let sites = match self.site_manager.send(crate::site::site_manager::SiteListRequest{}).wait() {
			Ok(s) => s.unwrap(),
			Err(_) => return Err(Error::CouldNotGetSiteList),
		};
		trace!("Sites: {}", sites.len());
		resp.params.sites = sites;
		// TODO: implement sitelist
		// get sites and send them in bunches of 100
		Ok(vec![resp])
	}
	fn handle_sitelist_response(&self, msg: &DiscoveryMessage) -> Result<Vec<DiscoveryMessage>, Error> {
		trace!("SiteListResponse from {:?} ({}:{}) sites: {}",
			msg.sender.peer_id,
			msg.sender.ip,
			msg.sender.port,
			msg.params.sites.len()
		);
		self.peer_manager.do_send(crate::peer::peer_manager::UpdatePeer{
			peer_id: msg.sender.peer_id.clone(),
			sites: msg.params.sites.iter().map(|a| a.to_vec()).collect(),
		});
		// TODO: implement sitelist
		// add peer to peer manager
		Ok(vec![])
	}
}
