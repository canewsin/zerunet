use std::collections::BTreeMap;
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
	fn connect() -> Self
	where
		Self: Sized;
	// Send data to connection
	fn send(&self, message: String) -> Result<(), ()>;
	// Stream file to connection without msgpacking
	// fn send_rawfile(&self, file_path: String, read_bytes: usize) -> Result<(),()>;
	// fn request(&self, cmd: String, params: BTreeMap<String, String>) -> Result<(),()>;
	// fn ping(&self) -> Result<(),()>;
}

pub fn connect(address: PeerAddress) -> Result<impl Connection, ()> {
	match address {
		PeerAddress::IPV4(ip, port) | PeerAddress::IPV6(ip, port) => {
			return Ok(TcpConnection::connect(&ip, port))
		}
		_ => return Err(()),
	}
}

pub struct TcpConnection {
	socket: TcpStream,
}

impl TcpConnection {
	pub fn connect(ip: &str, port: usize) -> TcpConnection {
		let socket = TcpStream::connect(format!("{}:{}", ip, port)).unwrap();
		TcpConnection { socket }
	}
}

impl Connection for TcpConnection {
	fn connect() -> TcpConnection {
		TcpConnection::connect("ip", 0)
	}
	fn send(&self, message: String) -> Result<(), ()> {
		Ok(())
	}
}
