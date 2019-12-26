mod error;
mod message;

use actix::{prelude::*, Actor, Context};
use log::*;
use std::net::UdpSocket;
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::collections::HashMap;

use crate::util::is_default;
use crate::site::site_manager::SiteManager;

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

pub fn start_local_discovery(site_manager: Addr<SiteManager>) -> Result<(), Error> {
	let port = BROADCAST_PORT;
	let mut local_ips = Vec::new();
	for mut iface in pnet::datalink::interfaces() {
		iface.ips.iter_mut().for_each(|ip| local_ips.push(*ip));
	}
	info!("Ips {:?}", &local_ips);
	let prob_ip = local_ips.iter().find(|ip| ip.to_string().starts_with("192"));
	if prob_ip.is_none() {
		error!("Could not find local ip!");
		return Err(Error::ErrorFindingIP);
	}
	let ip = prob_ip.unwrap().ip().to_string();
	trace!("Listening on {}:{}", &ip, port);
	let socket = UdpSocket::bind(format!("{}:{}", &ip, port))?;

	let system = actix::System::new("Local discovery server");
	let lds = LocalDiscoveryServer::new(ip, site_manager)?;
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

	system.run(); // TODO: necessary?
	Ok(())
}

fn broadcast_listen(socket: &UdpSocket) -> Result<DiscoveryMessage, Error> {
	let mut buf = [0u8; 4800]; // = 32bytes * 100sites + 1600reserve
	println!("Listening...");
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
}

impl LocalDiscoveryServer {
	pub fn new(ip: String, site_manager: Addr<SiteManager>) -> Result<LocalDiscoveryServer, Error> {
		let socket = UdpSocket::bind(format!("{}:{}",&ip, BROADCAST_PORT+1))?;
		let vec: Vec<u8> = (0..12).map(|_| rand::random::<u8>()).collect();
		// TODO: Bittorrent style id "-UT3530-%s" % CryptHash.random(12, "base64")
		let peer_id = format!("-UT3530-{}", base64::encode(&vec));
		info!("Generated id: {}", peer_id);
		let sender = Sender {
			service: String::from("zeronet"),
			peer_id,
			ip: ip.clone(),
			port: 11692,
			broadcast_port: BROADCAST_PORT,
			rev: 4241,
		};
		Ok(LocalDiscoveryServer {
			listen_ip: ip,
			listen_port: BROADCAST_PORT,
			socket,
			sender,
			site_manager,
		})
	}
	fn send(&self, addr: String, msg: DiscoveryMessage) -> Result<(), Error> {
		info!("Sending {} message to {}", msg.cmd, addr);
		let bytes = rmp_serde::to_vec_named(&msg)?;
		self.socket.send_to(&bytes, addr)?;
		Ok(())
	}
	pub fn broadcast(&self) -> Result<(), Error> {
		let msg = DiscoveryMessage::new(self.sender.clone(), LocalDiscoveryCommand::DiscoverRequest);
		trace!("Broadcasting {:?}", msg);
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
		info!("Incoming message {}", msg.cmd);
		let addr = format!("{}:{}", &msg.sender.ip, &msg.sender.broadcast_port);
		let response = match msg.cmd.as_str() {
			"discoverRequest" => self.handle_discovery_request(msg),
			"discoverResponse" => self.handle_discovery_response(msg),
			"siteListRequest" => self.handle_sitelist_request(msg),
			"siteListResponse" => self.handle_sitelist_response(msg),
			_ => return Err(Error::UnknownDiscoveryCommand),
		}?;

		match response {
			Some(resp) => self.send(addr, resp),
			None => Ok(()),
		}
	}
	fn handle_discovery_request(&self, msg: DiscoveryMessage) -> Result<Option<DiscoveryMessage>, Error> {
		let sites_changed: Vec<String> = vec![];
		let mut resp = DiscoveryMessage::new(self.sender.clone(), LocalDiscoveryCommand::DiscoverResponse);
		// TODO:: implement discovery
		// get date sites last updated
		resp.add_param("sites_changed", json!(sites_changed));
		Ok(Some(resp))
	}
	fn handle_discovery_response(&self, msg: DiscoveryMessage) -> Result<Option<DiscoveryMessage>, Error> {
		let resp = DiscoveryMessage::new(self.sender.clone(), LocalDiscoveryCommand::SiteListRequest);
		Ok(Some(resp))
	}
	fn handle_sitelist_request(&self, msg: DiscoveryMessage) -> Result<Option<DiscoveryMessage>, Error> {
		let resp = DiscoveryMessage::new(self.sender.clone(), LocalDiscoveryCommand::SiteListResponse);
		// TODO: implement sitelist
		// get sites and send them in bunches of 100
		Ok(Some(resp))
	}
	fn handle_sitelist_response(&self, msg: DiscoveryMessage) -> Result<Option<DiscoveryMessage>, Error> {
		info!("SiteListResponse from {:?}", msg.sender.peer_id);
		// TODO: implement sitelist
		// add peer to peer manager
		Ok(None)
	}
}
