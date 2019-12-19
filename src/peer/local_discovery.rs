use actix::{Actor, Context};
use log::*;
use std::net::UdpSocket;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::json;
use derive_more::Display;
use crate::error::Error;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
struct Sender {
	service: String,
	peer_id: String,
	port: usize,
	broadcast_port: usize,
	rev: usize,
	#[serde(skip)]
	ip: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DiscoveryMessage {
	params: HashMap<String, serde_json::Value>,
	#[serde(default)]
	sender: Sender,
	// TODO: cmd should be an enum, but mrp does not
	// serialize/deserialize those correctly
	cmd: String, 
}

use LocalDiscoveryCommand::*;
impl DiscoveryMessage {
	pub fn new(sender: Sender, cmd: LocalDiscoveryCommand) -> DiscoveryMessage {
		DiscoveryMessage {
			sender,
			cmd: cmd.to_string(),
			params: HashMap::new(),
		}
	}
	// Insert a parameter into the message
	pub fn add_param(&mut self, name: &str, value: serde_json::Value) {
		self.params.insert(name.to_string(), value);
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum LocalDiscoveryCommand {
	DiscoverRequest,
	SiteListRequest,
	DiscoverResponse,
	SiteListResponse,
}

impl ToString for LocalDiscoveryCommand {
	fn to_string(&self) -> String {
		match self {
			DiscoverRequest => "discoverRequest",
			SiteListRequest => "siteListRequest",
			DiscoverResponse => "discoverResponse",
			SiteListResponse => "siteListResponse",
		}.to_string()
	}
}

pub struct LocalDiscoveryServer {
	listen_port: usize,
	listen_ip: String,
	socket: UdpSocket,
	sender: Sender,
}

fn create_broadcast_socket(ip: &str, port: usize) -> Result<UdpSocket, Error> {
	let socket = match UdpSocket::bind(format!("{}:{}", ip, port)) {
		Ok(s) => s,
		Err(_) => {
			error!("Error binding discovery socket");
			return Err(Error::MissingError);
		}
	};
	Ok(socket)
}

impl LocalDiscoveryServer {
	pub fn new(ip: String, port: usize) -> Result<LocalDiscoveryServer, Error> {
		let mut local_ips = Vec::new();
		for mut iface in pnet::datalink::interfaces() {
			iface.ips.iter_mut().for_each(|ip| local_ips.push(*ip));
		}
		info!("Ips {:?}", &local_ips);
		let prob_ip = local_ips.iter().find(|ip| ip.to_string().starts_with("192"));
		if prob_ip.is_none() {
			return Err(Error::MissingError);
		}
		let ip = prob_ip.unwrap().ip().to_string();
		info!("Broadcasting on {}", &ip);
		let socket = create_broadcast_socket(&ip, port)?;
		let sender = Sender {
			service: String::from("zeronet"),
			// TODO: Bittorrent style id "-UT3530-%s" % CryptHash.random(12, "base64")
			peer_id: String::from("-UT3530-0042118A1977"),
			ip: ip.clone(),
			port: 11692,
			broadcast_port: port,
			rev: 4241,
		};
		Ok(LocalDiscoveryServer {
			listen_ip: ip,
			listen_port: port,
			socket,
			sender,
		})
	}
	pub fn listen(&self) -> Result<(), Error> {
		let mut buf = [0u8; 1024];
		println!("Listening...");
		let (amt, addr) = self.socket.recv_from(&mut buf).unwrap();
		let filled_buf = &mut buf[..amt];

		let mut msg: DiscoveryMessage = rmp_serde::from_read_ref(&filled_buf.to_vec())?;

		msg.sender.ip = format!("{}", addr.ip());

		// if &msg.sender.service != "zeronet" {
		// 	return Err(Error::MissingError);
		// }
		// if &msg.sender.peer_id == &self.sender.peer_id {
		// 	return Err(Error::MissingError);
		// }
		self.handle_message(msg);
		Ok(())
	}
	fn send(&self, addr: String, msg: DiscoveryMessage) -> Result<(), Error> {
		info!("Sending {} message to {}", msg.cmd, addr);
		let bytes = rmp_serde::to_vec_named(&msg)?;
		self.socket.send_to(&bytes, addr)?;
		Ok(())
	}
	pub fn broadcast(&self) -> Result<(), Error> {
		let msg = DiscoveryMessage::new(self.sender.clone(), LocalDiscoveryCommand::DiscoverRequest);
		info!("Broadcasting {:?}", msg);
		let bytes = rmp_serde::to_vec_named(&msg)?;
		let addr = format!("255.255.255.255:{}", self.listen_port);
		self.socket.send_to(&bytes, addr)?;
		Ok(())
	}
	fn handle_message(&self, msg: DiscoveryMessage) {
		println!("{:?}", msg);
		match msg.cmd.as_str() {
			"discoverRequest" => self.handle_discovery_request(msg),
			"discoverResponse" => self.handle_discovery_response(msg),
			"sitelistRequest" => self.handle_sitelist_request(msg),
			"sitelistResponse" => self.handle_sitelist_response(msg),
			_ => {},
		}
	}
	fn handle_discovery_request(&self, msg: DiscoveryMessage) {
		let addr = format!("{}:{}", &msg.sender.ip, &msg.sender.broadcast_port);

		let sites_changed: Vec<String> = vec![];
		let mut resp = DiscoveryMessage::new(self.sender.clone(), LocalDiscoveryCommand::DiscoverResponse);
		resp.add_param("sites_changed", json!(sites_changed));
		self.send(addr, resp);
	}
	fn handle_discovery_response(&self, msg: DiscoveryMessage) {
		let addr = format!("{}:{}", &msg.sender.ip, &msg.sender.broadcast_port);
	}
	fn handle_sitelist_request(&self, msg: DiscoveryMessage) {
		let addr = format!("{}:{}", &msg.sender.ip, &msg.sender.broadcast_port);
	}
	fn handle_sitelist_response(&self, msg: DiscoveryMessage) {
		let addr = format!("{}:{}", &msg.sender.ip, &msg.sender.broadcast_port);
	}
}
