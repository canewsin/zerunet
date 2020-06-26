use super::PeerMessage;
use crate::error::Error;
use log::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

#[derive(Debug)]
pub enum PeerAddress {
	IPV4(String, usize),
	IPV6(String, usize),
	Onion(String),
	I2P(String),
	Loki(String),
}

pub trait Connection<T: DeserializeOwned + Serialize> {
	// Send data to connection
	fn send(&mut self, message: T) -> Result<(), Error>;
	// Stream file to connection without msgpacking
	// fn send_rawfile(&self, file_path: String, read_bytes: usize) -> Result<(),()>;
	// TODO: Split up PeerMessage into Request and Response types? Or just have this one use send?
	fn request(&mut self, message: T) -> Result<T, Error>;
	// fn ping(&self) -> Result<(),()>;
	fn recv(&mut self) -> Result<T, Error>;
}

impl PeerAddress {
	pub fn connect(&self) -> Result<Box<dyn Connection<PeerMessage>>, ()> {
		match self {
			PeerAddress::IPV4(ip, port) | PeerAddress::IPV6(ip, port) => {
				return Ok(Box::new(TcpConnection::connect(&ip, *port)?))
			}
			_ => return Err(()),
		}
	}
}

#[derive(Serialize, Deserialize)]
pub struct Handshake {
	crypt: Option<String>,
	crypt_supported: Vec<String>,
	fileserver_port: usize,
	protocol: String,
	port_opened: bool,
	peer_id: String,
	rev: usize,
	target_ip: String,
	version: String,
	zerunet: bool,
}

impl Default for Handshake {
	fn default() -> Self {
		Handshake {
			crypt: None,
			crypt_supported: vec![],
			fileserver_port: 0,
			protocol: "v2".to_string(),
			port_opened: false,
			peer_id: "-ZHS".to_string(),
			rev: 0,
			target_ip: "127.0.0.1".to_string(),
			version: "0.7.1".to_string(),
			zerunet: true,
		}
	}
}

pub struct TcpConnection {
	socket: TcpStream,
}

impl TcpConnection {
	pub fn connect(ip: &str, port: usize) -> Result<TcpConnection, ()> {
		trace!("Connecting to {}:{}", &ip, port);
		let socket = TcpStream::connect(format!("{}:{}", ip, port));
		if socket.is_err() {
			error!("Could not connect to {}:{}", ip, port);
			return Err(());
		}
		let socket = socket.unwrap();
		socket.set_read_timeout(Some(Duration::new(60, 0))).unwrap();

		let mut connection = TcpConnection { socket };

		let mut handshake = PeerMessage::default();
		handshake.cmd = "handshake".to_string();
		handshake.params = serde_json::to_value(Handshake::default()).unwrap();
		handshake.req_id = Some(0);

		connection.request(handshake);

		Ok(connection)
	}
}

impl<T: DeserializeOwned + Serialize> Connection<T> for TcpConnection {
	fn send(&mut self, msg: T) -> Result<(), Error> {
		let bytes = rmp_serde::to_vec_named(&msg)?;
		let json_value: serde_json::Value = rmp_serde::from_slice(&bytes).unwrap();
		trace!("Writing {} to socket.", json_value);
		rmp_serde::encode::write_named(&mut self.socket, &msg);
		self.socket.write_all(&bytes)?;
		trace!("Wrote {:?} to socket.", String::from_utf8_lossy(&bytes));
		Ok(())
	}
	fn request(&mut self, msg: T) -> Result<T, Error> {
		self.send(msg)?;
		self.recv()
	}
	fn recv(&mut self) -> Result<T, Error> {
		let response: Result<T, _> = rmp_serde::from_read(&mut self.socket);
		match response {
			Ok(msg) => {
				trace!("{:?}", json!(&msg));
				Ok(msg)
			}
			Err(err) => {
				error!("Encountered error receiving response {:?}", err);
				Err(Error::MissingError)
			}
		}
	}
}
