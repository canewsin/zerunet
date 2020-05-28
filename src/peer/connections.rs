use super::PeerMessage;
use crate::error::Error;
use log::*;
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use serde::de::DeserializeOwned;
use serde::Serialize;

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

impl <T: DeserializeOwned + Serialize> Connection<T> for TcpConnection {
	fn send(&mut self, msg: T) -> Result<(), Error> {
		let bytes = rmp_serde::to_vec_named(&msg)?;
		let json_value: serde_json::Value = rmp_serde::from_slice(&bytes).unwrap();
		trace!("Writing {} to socket.", json_value);
		self.socket.write_all(&bytes)?;
		trace!("Wrote \"{:?}\" to socket.", String::from_utf8_lossy(&bytes));
		Ok(())
	}
	fn request(&mut self, msg: T) -> Result<T, Error> {
		self.send(msg)?;
		self.recv()
	}
	fn recv(&mut self) -> Result<T, Error> {
		let response: Result<T, _> = rmp_serde::from_read(&mut self.socket);
		match response {
			Ok(msg) => Ok(msg),
			Err(err) => {
				error!("Encountered error receiving response {:?}", err);
				Err(Error::MissingError)
			},
		}
	}
}
