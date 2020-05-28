use super::PeerMessage;
use crate::error::Error;
use log::*;
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub enum PeerAddress {
	IPV4(String, usize),
	IPV6(String, usize),
	Onion(String),
	I2P(String),
	Loki(String),
}

pub trait Connection {
	// Send data to connection
	fn send(&mut self, message: PeerMessage) -> Result<PeerMessage, Error>;
	// Stream file to connection without msgpacking
	// fn send_rawfile(&self, file_path: String, read_bytes: usize) -> Result<(),()>;
	// TODO: Split up PeerMessage into Request and Response types? Or just have this one use send?
	// fn request(&self, cmd: String, params: BTreeMap<String, String>) -> Result<(),()>;
	// fn ping(&self) -> Result<(),()>;
	fn message_loop(&mut self) -> Result<PeerMessage, Error>;
}

impl PeerAddress {
	pub fn connect(&self) -> Result<Box<dyn Connection>, ()> {
		match self {
			PeerAddress::IPV4(ip, port) | PeerAddress::IPV6(ip, port) => {
				return Ok(Box::new(TcpConnection::connect(&ip, *port)?))
			}
			_ => return Err(()),
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
			return Err(())
		}
		let socket = socket.unwrap();
		socket
			.set_read_timeout(Some(std::time::Duration::new(1, 0)))
			.unwrap();

		Ok(TcpConnection { socket })
	}
}

impl Connection for TcpConnection {
	fn send(&mut self, msg: PeerMessage) -> Result<PeerMessage, Error> {
		let bytes = rmp_serde::to_vec_named(&msg)?;
		self.socket.write_all(&bytes)?;
		trace!("Wrote \"{:?}\" to socket.", String::from_utf8_lossy(&bytes));
		self.message_loop()
	}
	fn message_loop(&mut self) -> Result<PeerMessage, Error> {
		let response: Result<PeerMessage, _> = rmp_serde::from_read(&mut self.socket);
		match response {
			Ok(msg) => Ok(msg),
			Err(err) => {
				error!("Encountered error receiving response {:?}", err);
				Err(Error::MissingError)
			}
		}
	}
}
