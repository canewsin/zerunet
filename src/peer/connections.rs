use std::net::TcpStream;

pub enum PeerAddr {
	IPV4(String, usize),
	IPV6(String, usize),
	Onion(String),
	I2P(String),
	Loki(String),
}

pub trait Connection {}

pub fn connect(address: PeerAddr) -> Result<impl Connection, ()> {
	match address {
		PeerAddr::IPV4(ip, port) | PeerAddr::IPV6(ip, port) => return Ok(TcpConnection::connect(&ip, port)),
		_ => return Err(()),
	}
}

pub struct TcpConnection {
	socket: TcpStream,
}

impl TcpConnection {
	pub fn connect(ip: &str, port: usize) -> TcpConnection {
		let socket = TcpStream::connect(format!("{}:{}", ip, port)).unwrap();
		TcpConnection {
			socket,
		}
	}
}

impl Connection for TcpConnection {

}