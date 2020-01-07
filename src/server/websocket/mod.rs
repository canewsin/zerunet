pub mod error;
pub mod request;
pub mod response;

use crate::site::site_manager::{Lookup, SiteManager};
use actix::{Actor, Addr, StreamHandler};
use actix_web::{
	web::{Data, Payload, Query},
	HttpRequest, HttpResponse, Result,
};
use actix_web_actors::ws;
use futures::executor::block_on;
use log::*;
use request::{Command, CommandType::*};
use response::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use error::Error;

pub async fn serve_websocket(
	req: HttpRequest,
	query: Query<HashMap<String, String>>,
	data: Data<crate::server::ZeroServer>,
	stream: Payload,
) -> Result<HttpResponse, actix_web::Error> {
	info!("Serving websocket");
	let wrapper_key = query.get("wrapper_key").unwrap();

	let future = data
		.site_manager
		.send(Lookup::Key(String::from(wrapper_key)));
	let (address, addr) = match block_on(future) {
		Ok(Ok(resp)) => resp,
		_ => {
			warn!("Websocket established, but wrapper key invalid");
			return Err(actix_web::Error::from(()));
		}
	};

	info!("Websocket established for {}", address.get_address_short());
	let mut websocket = ZeruWebsocket {
		site_manager: data.site_manager.clone(),
		site_addr: addr,
		address: address,
	};

	let resp = ws::start(websocket, &req, stream);
	resp
}

struct ZeruWebsocket {
	site_manager: Addr<SiteManager>,
	site_addr: actix::Addr<crate::site::Site>,
	address: crate::site::address::Address,
}

impl Actor for ZeruWebsocket {
	type Context = ws::WebsocketContext<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {}
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ZeruWebsocket {
	fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
		if msg.is_err() {
			error!("Protocol error on websocket message");
			return;
		}
		match msg.unwrap() {
			ws::Message::Ping(msg) => ctx.pong(&msg),
			ws::Message::Text(text) => {
				let command: Command = match serde_json::from_str(&text) {
					Ok(c) => c,
					Err(e) => {
						error!(
							"Could not deserialize incoming message: {:?} ({:?})",
							text, e
						);
						return;
					}
				};
				if let Err(err) = self.handle_command(ctx, &command) {
					handle_error(ctx, command, format!("{:?}", err));
					return;
				}
			}
			ws::Message::Binary(bin) => ctx.binary(bin),
			_ => (),
		}
	}
}

#[derive(Serialize, Deserialize)]
pub struct WrapperCommand {
	cmd: WrapperCommandType,
	to: isize,
	result: WrapperResponse,
}

#[derive(Serialize, Deserialize)]
pub enum WrapperResponse {
	Empty,
	ServerInfo(ServerInfo),
	Text(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WrapperCommandType {
	Response,
	Error,
	WrapperReady,
	Ping,
	WrapperOpenedWebsocket,
	WrapperClosedWebsocket,
}

#[derive(Serialize, Deserialize)]
pub struct ServerPortOpened {
	ipv4: bool,
	ipv6: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ServerInfo {
	ip_external: bool,
	port_opened: ServerPortOpened,
	platform: String,
	fileserver_ip: String,
	fileserver_port: usize,
	tor_enabled: bool,
	tor_status: String,
	tor_has_meek_bridges: bool,
	ui_ip: String,
	ui_port: usize,
	version: String,
	rev: usize,
	timecorrection: f64,
	language: String,
	debug: bool,
	offline: bool,
	plugins: Vec<String>,
	plugins_rev: HashMap<String, usize>,
	// user_settings
}

fn handle_ping(
	ctx: &mut ws::WebsocketContext<ZeruWebsocket>,
	req: &Command,
) -> Result<Message, Error> {
	trace!("Handling ping");
	let pong = String::from("pong");
	req.respond(pong)
}

fn handle_server_info(
	ctx: &mut ws::WebsocketContext<ZeruWebsocket>,
	req: &Command,
) -> Result<Message, Error> {
	warn!("Handling ServerInfo request");
	let server_info = ServerInfo {
		ip_external: false,
		port_opened: ServerPortOpened {
			ipv4: true,
			ipv6: false,
		},
		platform: String::from("linux"), // TODO: get actual platform
		fileserver_ip: String::from(super::SERVER_URL),
		fileserver_port: super::SERVER_PORT,
		tor_enabled: false,
		tor_status: String::from("Disabled"), // TODO: get actual tor status
		tor_has_meek_bridges: false,
		ui_ip: String::from("localhost"), // TODO: get actual ui ip
		ui_port: super::SERVER_PORT,
		version: String::from("0.7.1"),
		rev: 4300,
		timecorrection: 0f64,
		language: String::from("en"),
		debug: true,
		offline: false,
		plugins: Vec::new(),
		plugins_rev: HashMap::new(),
		// user_settings:
	};
	req.respond(server_info)
}

fn handle_error(
	ctx: &mut ws::WebsocketContext<ZeruWebsocket>,
	command: Command,
	text: String,
) -> Result<(), actix_web::Error> {
	let error = WrapperCommand {
		cmd: WrapperCommandType::Error,
		to: command.id,
		result: WrapperResponse::Text(text),
	};
	let j = serde_json::to_string(&error)?;
	ctx.text(j);
	Ok(())
}

impl ZeruWebsocket {
	fn handle_command(
		&mut self,
		ctx: &mut ws::WebsocketContext<ZeruWebsocket>,
		command: &Command,
	) -> Result<(), Error> {
		let response = match command.cmd {
			Ping => handle_ping(ctx, command),
			ServerInfo => handle_server_info(ctx, command),
			SiteInfo => {
				warn!("Handling SiteInfo request with dummy response");
				let site_info_req = crate::site::SiteInfoRequest {};
				let result = block_on(self.site_addr.send(site_info_req));
				// TODO: Clean up this part
				if result.is_err() {
					return Err(Error {});
				}
				let result = result.unwrap();
				if result.is_err() {
					return Err(Error {});
				}
				let result = result.unwrap();
				command.respond(result)
			}
			ServerErrors => {
				warn!("Handling ServerErrors request with dummy response");
				// TODO: actually return the errors
				let errors: Vec<Vec<String>> = vec![];
				command.respond(errors)
			}
			AnnouncerStats => {
				warn!("Handling AnnouncerStats request with dummy response");
				// TODO: actually return announcer stats
				let mut stats: HashMap<String, _> = HashMap::new();
				stats.insert(
					String::from("zero://boot3rdez4rzn36x.onion:15441"),
					crate::tracker::AnnouncerStats {
						status: String::from("announced"),
						num_request: 0,
						num_success: 0,
						num_error: 0,
						time_request: 0.0,
						time_last_error: 0.0,
						time_status: 0.0,
						last_error: String::from("Not implemented yet"),
					},
				);
				command.respond(stats)
			}
			UserGetSettings => {
				warn!("Handling UserGetSettings with dummy response");
				// TODO: actually return user settings
				command.respond(String::new())
			}
			SiteList => {
				info!("Handling SiteList");
				// TODO: actually return list of sites
				let sites = block_on(
					self
						.site_manager
						.send(crate::site::site_manager::SiteInfoListRequest {}),
				)
				.unwrap()
				.unwrap();
				command.respond(sites)
			}
			OptionalLimitStats => {
				// TODO: replace dummy response with actual response
				warn!("Handling OptionalLimitStats with dummy response");
				let limit_stats = crate::optional_files::OptionalLimitStats {
					limit: String::from("10%"),
					used: 1000000,
					free: 4000000,
				};
				command.respond(limit_stats)
			}
			FeedQuery => {
				warn!("Handling FeedQuery");
				// TODO: move this to proper place
				#[derive(Serialize, Deserialize, Debug)]
				struct FeedQueryResponse {
					rows: Vec<String>,
				}
				let result = FeedQueryResponse { rows: Vec::new() };
				command.respond(result)
			}
			FileGet => {
				warn!("Handling FileGet request");
				// if required || inner_path in site.bad_files
				// if let Some(addr) = addr {
				// 	addr.send(FileNeed command);
				// }
				let msg: crate::site::FileGetRequest = match serde_json::from_value(command.params.clone())
				{
					Ok(m) => m,
					Err(e) => {
						error!("{:?}", e);
						// TODO: error
						crate::site::FileGetRequest::default()
					}
				};
				let mut path = PathBuf::from("../ZeroNet/data/");
				path.push(Path::new(&format!(
					"{}/{}",
					self.address.to_string(),
					msg.inner_path
				)));
				let mut file = match File::open(path) {
					Ok(f) => f,
					Err(err) => {
						error!("Failed to get file: {:?}", err);
						return Err(Error {}); // TODO: respond with 404 equivalent
					}
				};
				let mut string = String::new();
				match file.read_to_string(&mut string) {
					Ok(_) => {}
					Err(_) => {
						error!("Failed to read file to string");
						return Err(Error {});
					} // TODO: respond with 404 equivalent
				}

				command.respond(string)
			}
			_ => {
				let cmd = command.cmd.clone();
				error!("Unhandled command: {:?}", cmd);
				return Err(Error {});
			}
		};

		let j = serde_json::to_string(&response?).unwrap();
		ctx.text(j);

		Ok(())
	}
}
