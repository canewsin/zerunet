use actix::{prelude::*, Actor, Context};
use log::*;
use std::net::UdpSocket;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::json;
use derive_more::Display;
use crate::error::Error;
use crate::util::is_default;
use crate::site::site_manager::SiteManager;

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(default)]
struct Params {
	sites_changed: usize,
	sites: Vec<serde_bytes::ByteBuf>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
struct DiscoveryMessage {
	#[serde(skip_serializing_if = "is_default")]
	// params: Params,
	params: HashMap<String, serde_json::Value>,
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
			// params: Params::default(),
			params: HashMap::new(),
		}
	}
	// Insert a parameter into the message
	pub fn add_param(&mut self, name: &str, value: serde_json::Value) {
		// self.params.insert(name.to_string(), value);
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
		return Err(Error::MissingError);
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
				Err(_) => {},
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
		let sender = Sender {
			service: String::from("zeronet"),
			// TODO: Bittorrent style id "-UT3530-%s" % CryptHash.random(12, "base64")
			peer_id: String::from("-UT3530-0042118A1977"),
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
			return Err(Error::MissingError);
		}
		if &msg.sender.peer_id == &self.sender.peer_id {
			return Err(Error::MissingError);
		}
		info!("Incoming message {}", msg.cmd);
		let addr = format!("{}:{}", &msg.sender.ip, &msg.sender.broadcast_port);
		let response = match msg.cmd.as_str() {
			"discoverRequest" => self.handle_discovery_request(msg),
			"discoverResponse" => self.handle_discovery_response(msg),
			"siteListRequest" => self.handle_sitelist_request(msg),
			"siteListResponse" => self.handle_sitelist_response(msg),
			_ => return Err(Error::MissingError),
		}?;
		self.send(addr, response)
	}
	fn handle_discovery_request(&self, msg: DiscoveryMessage) -> Result<DiscoveryMessage, Error> {
		let sites_changed: Vec<String> = vec![];
		let mut resp = DiscoveryMessage::new(self.sender.clone(), LocalDiscoveryCommand::DiscoverResponse);
		resp.add_param("sites_changed", json!(sites_changed));
		Ok(resp)
	}
	fn handle_discovery_response(&self, msg: DiscoveryMessage) -> Result<DiscoveryMessage, Error> {
		let resp = DiscoveryMessage::new(self.sender.clone(), LocalDiscoveryCommand::SiteListRequest);
		Ok(resp)
	}
	fn handle_sitelist_request(&self, msg: DiscoveryMessage) -> Result<DiscoveryMessage, Error> {
		let resp = DiscoveryMessage::new(self.sender.clone(), LocalDiscoveryCommand::SiteListResponse);
		Ok(resp)
	}
	fn handle_sitelist_response(&self, msg: DiscoveryMessage) -> Result<DiscoveryMessage, Error> {
		info!("{:?}", msg.params);
		Err(Error::MissingError)
	}
}
