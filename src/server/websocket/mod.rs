pub mod error;
pub mod request;
pub mod response;
mod handlers;

use crate::site::site_manager::{Lookup, SiteManager};
use crate::user::user_manager::{UserManager, UserRequest};
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
		user_manager: data.user_manager.clone(),
		site_addr: addr,
		address: address,
		data_path: data.data_path.clone(),
	};

	let resp = ws::start(websocket, &req, stream);
	resp
}

pub struct ZeruWebsocket {
	site_manager: Addr<SiteManager>,
	user_manager: Addr<UserManager>,
	site_addr: actix::Addr<crate::site::Site>,
	address: crate::site::address::Address,
	data_path: PathBuf,
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
	multiuser: bool,
	master_address: String,
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
		plugins: vec![String::from("Multiuser")],
		plugins_rev: HashMap::new(),
		multiuser: true, // TODO: make setting
		master_address: String::from("TestAddress"), // TODO: get actual master address
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
			SiteInfo => handlers::sites::handle_site_info(self, ctx, command),
			SiteList => handlers::sites::handle_site_list(self, ctx, command),
			OptionalLimitStats => handlers::sites::handle_optional_limit_stats(self, ctx, command),
			FileGet => handlers::files::handle_file_get(self, ctx, command),
			UserGetSettings => handlers::users::handle_user_get_settings(self, ctx, command),
			UserGetGlobalSettings => handlers::users::handle_user_get_global_settings(self, ctx, command),
			AnnouncerStats => handlers::trackers::handle_announcer_stats(self, ctx, command),
			ChannelJoin => handlers::sites::handle_channel_join(self, ctx, command),
			ChannelJoinAllsite => handlers::sites::handle_channel_join_all_site(self, ctx, command),
			ServerErrors => {
				warn!("Handling ServerErrors request with dummy response");
				// TODO: actually return the errors
				let errors: Vec<Vec<String>> = vec![];
				command.respond(errors)
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
