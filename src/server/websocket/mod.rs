pub mod request;
pub mod response;

use crate::site::site_manager::{Lookup, SiteManager};
use actix::{Actor, Addr, StreamHandler};
use actix_web::{
	web::{Data, Payload, Query},
	Error, HttpRequest, HttpResponse, Result,
};
use actix_web_actors::ws;
use futures::Future;
use log::*;
use request::{Command, CommandType::*};
use response::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ws::WebsocketContext;

pub fn serve_websocket(
	req: HttpRequest,
	query: Query<HashMap<String, String>>,
	data: Data<crate::server::ZeroServer>,
	stream: Payload,
) -> Result<HttpResponse, Error> {
	info!("Serving websocket");
	let wrapper_key = query.get("wrapper_key").unwrap();

	let mut websocket = ZeruWebsocket {
		site_manager: data.site_manager.clone(),
		site_addr: None,
	};

	let future = data
		.site_manager
		.send(Lookup::Key(String::from(wrapper_key)));
	match future.wait() {
		Ok(Ok((address, addr))) => {
			websocket.site_addr = Some(addr);
			info!("Websocket established for {}", address);
		}
		_ => warn!("Websocket established, but wrapper key invalid"),
	}

	let resp = ws::start(websocket, &req, stream);
	resp
}

struct ZeruWebsocket {
	site_manager: Addr<SiteManager>,
	site_addr: Option<actix::Addr<crate::site::Site>>,
}

impl Actor for ZeruWebsocket {
	type Context = ws::WebsocketContext<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {}
}

impl StreamHandler<ws::Message, ws::ProtocolError> for ZeruWebsocket {
	fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
		match msg {
			ws::Message::Ping(msg) => ctx.pong(&msg),
			ws::Message::Text(text) => {
				let command: Command = match serde_json::from_str(&text) {
					Ok(c) => c,
					Err(_) => {
						error!("Could not deserialize incoming message: {:?}", text);
						return;
					}
				};
				handle_command(ctx, self.site_addr.clone(), command);
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
pub struct ServerInfo {
	debug: bool,
	fileserver_ip: String,
	fileserver_port: usize,
	ip_external: bool,
	platform: String,
	ui_ip: String,
	ui_port: usize,
	version: String,
}

fn handle_ping(ctx: &mut ws::WebsocketContext<ZeruWebsocket>, req: &Command) -> Result<(), Error> {
	trace!("Handling ping");
	let pong = String::from("pong");
	let resp = Message::respond(req, pong)?;
	let j = serde_json::to_string(&resp)?;
	ctx.text(j);
	Ok(())
}

fn handle_server_info(
	ctx: &mut ws::WebsocketContext<ZeruWebsocket>,
	req: &Command,
) -> Result<(), Error> {
	trace!("Handle ServerInfo request");
	let server_info = ServerInfo {
		debug: true,
		fileserver_ip: String::from(super::SERVER_URL),
		fileserver_port: super::SERVER_PORT,
		ip_external: false,
		platform: String::from("linux64"),
		ui_ip: String::from("localhost"),
		ui_port: super::SERVER_PORT,
		version: String::from("0.0.1"),
	};
	let resp = Message::respond(req, server_info)?;

	let j = serde_json::to_string(&resp)?;
	ctx.text(j);
	Ok(())
}

fn handle_error(ctx: &mut ws::WebsocketContext<ZeruWebsocket>, text: String) -> Result<(), Error> {
	let error = WrapperCommand {
		cmd: WrapperCommandType::Error,
		to: 0,
		result: WrapperResponse::Text(text),
	};
	let j = serde_json::to_string(&error)?;
	ctx.text(j);
	Ok(())
}

fn handle_command(
	ctx: &mut ws::WebsocketContext<ZeruWebsocket>,
	addr: Option<actix::Addr<crate::site::Site>>,
	command: Command,
) {
	match command.cmd {
		ServerInfo => {
			handle_server_info(ctx, &command);
		}
		Ping => {
			// ctx.spawn(|c| {
			handle_ping(ctx, &command);
			// });
		}
		SiteInfo => {
			if let Some(addr) = addr {
				let site_info_req = crate::site::SiteInfoRequest {};
				let result = addr.send(site_info_req).wait();
				if let Ok(Ok(res)) = result {
					let resp = Message::respond(&command, res).unwrap();

					let j = serde_json::to_string(&resp).unwrap();
					ctx.text(j);
				}
			}
		}
		_ => {
			error!("Unhandled command: {:?}", command.cmd);
			handle_error(ctx, format!("Unhandled command: {:?}", command.cmd));
		}
	};
	// match command.cmd {
	//   UserGetGlobalSettings => info!("userGetGlobal"),
	//   // ChannelJoin => info!("channelJoin"),
	//   // SiteInfo => info!("siteInfo"),
	//   _ => error!("Unknown command: '{:?}'", command.cmd),
	// }
}
