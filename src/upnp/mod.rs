use crate::error::Error;
use derive_more::Display;
use quick_xml::{events::Event, Reader};
use regex::Regex;
use reqwest::get;
use std::net::UdpSocket;
use std::time::Duration;

pub struct UPnBrunch {
	host_url: String,
	upnp_schema: Connection,
	control_path: String,
	protocols: Vec<Protocol>,
	retries: usize,
	local_ips: Vec<ipnetwork::IpNetwork>,
}

#[derive(Display)]
pub enum Protocol {
	TCP,
	UDP,
}

#[derive(Display)]
pub enum Connection {
	WANIPConnection,
	WANPPPConnection,
}

const MULTICAST_ADDRESS: &'static str = "239.255.255.250:1900";
const SEARCH_REQUEST: &'static str = "M-SEARCH * HTTP/1.1\r
HOST: 239.255.255.250:1900\r
MAN: \"ssdp:discover\"\r
MX: 2\r
ST: urn:schemas-upnp-org:device:InternetGatewayDevice:1\r\n\r\n";

// Performs an M-SEARCH and returns the response
pub fn m_search() -> Result<String, Error> {
	let socket = UdpSocket::bind("[::]:0").unwrap();
	println!("Sending search request");
	socket
		.send_to(SEARCH_REQUEST.as_bytes(), MULTICAST_ADDRESS)
		.expect("Couldn't send data");
	socket.set_read_timeout(Some(Duration::new(5, 0)))?;

	let mut buf = [0; 1024];
	let (amt, _src) = socket.recv_from(&mut buf)?;
	let filled_buf = &mut buf[..amt];
	let s = String::from_utf8(filled_buf.to_vec()).unwrap();

	Ok(s)
}

// Retrieves the location from a SSDP response
pub fn retrieve_location(resp: String) -> Option<String> {
	let re = Regex::new(r"(?P<name>.*?): (?P<value>.*?)\r\n").unwrap();
	let mut captures = re.captures_iter(&resp);
	captures
		.find(|c| &c[1].to_uppercase() == "LOCATION")
		.map(|c| String::from(&c[2]))
}

// Retrieves the device's UPnP profile and parses it
// for the controlURL of either a WANIPConnection
// or WANPPPConnection.
pub fn retrieve_igd_profile(url: &String) -> Result<(String, Connection), Error> {
	let mut resp: reqwest::Response = get(url)?;
	let text = resp.text()?;
	let mut reader = Reader::from_str(&text);
	let mut buf = Vec::new();
	let mut control_url = None;
	let mut upnp_schema = None;
	loop {
		match reader.read_event(&mut buf) {
			Ok(Event::Start(ref e)) => {
				if e.name() == b"controlURL" {
					let s = reader.read_text(e.name(), &mut Vec::new()).unwrap();
					control_url = Some(s);
					if upnp_schema.is_some() {
						break;
					}
				}
				if e.name() == b"serviceType" {
					let s = reader.read_text(e.name(), &mut Vec::new()).unwrap();
					if s.find("WANIPConnection").is_some() {
						upnp_schema = Some(Connection::WANIPConnection);
					}
					if s.find("WANPPPConnection").is_some() {
						upnp_schema = Some(Connection::WANPPPConnection);
					}
				}
			}
			Ok(Event::Eof) => break,
			_ => (),
		}
	}
	if control_url.is_none() || upnp_schema.is_none() {
		return Err(Error::MissingError);
	}
	Ok((control_url.unwrap(), upnp_schema.unwrap()))
}

pub fn create_open_message(
	port: usize,
	protocol: String,
	host_ip: String,
	description: String,
	upnp_schema: &Connection,
) -> String {
	let mut message = format!("<?xml version=\"1.0\"?>
	<s:Envelope xmlns:s=\"http://schemas.xmlsoap.org/soap/envelope/\" s:encodingStyle=\"http://schemas.xmlsoap.org/soap/encoding/\">
			<s:Body>
					<u:AddPortMapping xmlns:u=\"urn:schemas-upnp-org:service:{upnp_schema}:1\">
							<NewRemoteHost></NewRemoteHost>
							<NewExternalPort>{port}</NewExternalPort>
							<NewProtocol>{protocol}</NewProtocol>
							<NewInternalPort>{port}</NewInternalPort>
							<NewInternalClient>{host_ip}</NewInternalClient>
							<NewEnabled>1</NewEnabled>
							<NewPortMappingDescription>{description}</NewPortMappingDescription>
							<NewLeaseDuration>0</NewLeaseDuration>
					</u:AddPortMapping>
			</s:Body>
	</s:Envelope>", upnp_schema=upnp_schema, port=port, protocol=protocol, host_ip=host_ip, description=description);
	message.retain(|c| c != '\t' && c != '\n');
	message
}

pub fn create_close_message(port: usize, protocol: String, upnp_schema: &Connection) -> String {
	let mut message = format!("<?xml version=\"1.0\"?>
	<s:Envelope xmlns:s=\"http://schemas.xmlsoap.org/soap/envelope/\" s:encodingStyle=\"http://schemas.xmlsoap.org/soap/encoding/\">
			<s:Body>
					<u:DeletePortMapping xmlns:u=\"urn:schemas-upnp-org:service:{upnp_schema}:1\">
							<NewRemoteHost></NewRemoteHost>
							<NewExternalPort>{port}</NewExternalPort>
							<NewProtocol>{protocol}</NewProtocol>
					</u:DeletePortMapping>
			</s:Body>
	</s:Envelope>
	", port=port, protocol=protocol, upnp_schema=upnp_schema);
	message.retain(|c| c != '\t' && c != '\n');
	message
}

struct SoapRequest {
	soap_fn: String,
	soap_msg: String,
}

enum MessageType {
	Open,
	Close,
}

impl UPnBrunch {
	pub fn new() -> Result<UPnBrunch, Error> {
		let mut local_ips = Vec::new();
		for mut iface in pnet::datalink::interfaces() {
			iface.ips.iter_mut().for_each(|ip| local_ips.push(*ip));
		}
		let resp = m_search()?;
		if let Some(loc) = retrieve_location(resp) {
			let (control_path, upnp_schema) = retrieve_igd_profile(&loc)?;
			println!("{}, {}", control_path, upnp_schema);
			let re = Regex::new(r"\d.*:\d*").unwrap();
			let host = re.captures(&loc).unwrap();
			let host_url = String::from(host.get(0).map_or("", |m| m.as_str()));
			let punch = UPnBrunch {
				host_url: host_url,
				local_ips: local_ips,
				control_path: control_path,
				upnp_schema: upnp_schema,
				protocols: Vec::new(),
				retries: 0,
			};
			return Ok(punch);
		}
		Err(Error::MissingError)
	}

	// Sets the number of retry attempts for this client
	pub fn retries(mut self, retries: usize) -> UPnBrunch {
		self.retries = retries;
		self
	}

	// Adds a protocol for this client
	pub fn add_protocol(mut self, protocol: Protocol) -> UPnBrunch {
		self.protocols.push(protocol);
		self
	}

	// Sends a request to the IGD
	fn send_soap_request(
		&self,
		soap_fn: &String,
		soap_message: &String,
	) -> Result<reqwest::Response, Error> {
		let client = reqwest::Client::new();
		let url_string = format!("http://{}{}", &self.host_url, &self.control_path);
		let url = reqwest::Url::parse(&url_string).unwrap();
		let resp = client
			.post(url)
			.header(
				"SOAPAction",
				format!(
					"\"urn:schemas-upnp-org:service:{schema}:1#{fn_name}\"",
					schema = self.upnp_schema,
					fn_name = soap_fn
				),
			)
			.header("Content-Type", "text/xml")
			.body(soap_message.clone())
			.send()?;
		Ok(resp)
	}

	// Sends requests for each protocol to the IGD
	fn send_requests(&self, requests: &mut Vec<SoapRequest>) -> Result<(), Error> {
		let mut responses = requests
			.iter_mut()
			.map(|r| self.send_soap_request(&r.soap_fn, &r.soap_msg).unwrap());
		if responses.all(|r| r.status().is_success()) {
			Ok(())
		} else {
			Err(Error::MissingError)
		}
	}

	// Creates the appropriate request messages and sends them to the IGD
	fn orchestrate_soap_request(
		&self,
		ip: &String,
		port: usize,
		msg_type: &MessageType,
		desc: &str,
	) -> Result<(), Error> {
		let mut soap_requests: Vec<SoapRequest> = self
			.protocols
			.iter()
			.map(|p| match &msg_type {
				MessageType::Open => SoapRequest {
					soap_fn: String::from("AddPortMapping"),
					soap_msg: create_open_message(
						port,
						format!("{}", p),
						ip.clone(),
						String::from(desc),
						&self.upnp_schema,
					),
				},
				MessageType::Close => SoapRequest {
					soap_fn: String::from("DeletePortMapping"),
					soap_msg: create_close_message(port, format!("{}", p), &self.upnp_schema),
				},
			})
			.collect();
		self.send_requests(&mut soap_requests)
	}

	// Builds messages and attempts to send them to the IGD
	fn communicate_with_igd(
		&self,
		port: usize,
		desc: &str,
		msg_type: MessageType,
	) -> Result<(), Error> {
		// TODO: prioritize likely candidates (192., 10.)
		for ip in &self.local_ips {
			let ip_s = format!("{}", &ip.ip());
			for _ in 0..(self.retries + 1) {
				if self
					.orchestrate_soap_request(&ip_s, port, &msg_type, desc)
					.is_ok()
				{
					return Ok(());
				}
			}
		}
		Err(Error::MissingError)
	}

	// Ask the IGD to open a port
	pub fn open_port(&self, port: usize, desc: &str) -> Result<(), Error> {
		self.communicate_with_igd(port, desc, MessageType::Open)
	}

	// Ask the IGD to close a port
	pub fn close_port(&self, port: usize) -> Result<(), Error> {
		self.communicate_with_igd(port, "", MessageType::Close)
	}
}

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod test {
	use super::{Protocol, UPnBrunch};

	#[test]
	pub fn test_upnbrunch() {
		let punch = UPnBrunch::new()
			.unwrap()
			.retries(3)
			.add_protocol(Protocol::TCP);
		let port = 15443;
		let desc = String::from("UPnPBrunch");

		assert!(punch.open_port(port, &desc).is_ok(), "Opening port");
		assert!(punch.close_port(port).is_ok(), "Closing port");
	}
}
