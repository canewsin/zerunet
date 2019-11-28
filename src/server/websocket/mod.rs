pub mod request;
pub mod response;

use actix::{Actor, StreamHandler};
use actix_web::{HttpRequest, Result, Error, HttpResponse};
use actix_web::web::Payload;
use actix_web_actors::ws;
use log::*;
use futures::Future;
use serde::{Serialize, Deserialize};
use crate::site;
use request::{
	CommandType::*,
	Command,
};
use response::Message;

pub fn serve_websocket(req: HttpRequest, stream: Payload) -> Result<HttpResponse, Error> {
  let site = site::Site::new();
  let addr: actix::Addr<site::Site> = site.start();
  info!("Serving websocket");

  let resp = ws::start(ZeruWebsocket {
    site_addr: addr,
  }, &req, stream);
  resp
}

struct ZeruWebsocket {
  site_addr: actix::Addr<site::Site>,
}

impl Actor for ZeruWebsocket {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    let msg = WrapperCommand{
      cmd: WrapperCommandType::WrapperOpenedWebsocket,
      to: 0,
      result: WrapperResponse::Empty,
    };
    let j = serde_json::to_string(&msg);
    if let Ok(text) = j {
      info!("{}", text);
      ctx.text(text);
    }
  }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for ZeruWebsocket {
  fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
    // info!("{:?}", msg);
    match msg {
      ws::Message::Ping(msg) => ctx.pong(&msg),
      ws::Message::Text(text) => {
        let result = self.site_addr
          .send(site::FileRequest(String::from("test")));
        actix::Arbiter::spawn(
          result.map(|res| {
            match res {
              Ok(result) => info!("got result {:?}", result),
              Err(err) => error!("got err {:?}", err),
            }
          }).map_err(|err| {
            warn!("Actor is probably dead {:?}", err);
          })
        );

        let command: Command = match serde_json::from_str(&text) {
          Ok(c) => c,
          Err(_) => {error!("Unknown command: {:?}", text); return},
        };
        match command.cmd {
          ServerInfo => { handle_server_info(ctx, &command); },
          SiteInfo => {
            let response = WrapperCommand {
              cmd: WrapperCommandType::Response,
              to: command.id,
              result: WrapperResponse::Text(String::from("")),
            };
            let j = serde_json::to_string(&response).unwrap();
            error!("SiteInfo improperly handled!");
            ctx.text(j);
          },
          InnerReady => { handle_inner_ready(ctx); },
          _ => {
            error!("Unhandled command: {:?}", command.cmd);
            handle_error(ctx, format!("Unhandled command: {:?}", command.cmd));
          },
        };
      },
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
#[serde(rename_all ="camelCase")]
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

fn handle_server_info(ctx: &mut ws::WebsocketContext<ZeruWebsocket>, req: &Command) -> Result<(), Error> {
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

fn handle_inner_ready(ctx: &mut ws::WebsocketContext<ZeruWebsocket>) -> Result<(), Error> {
  trace!("Handle InnerReady message");
  let opened = WrapperCommand {
    cmd: WrapperCommandType::WrapperOpenedWebsocket,
    to: 0,
    result: WrapperResponse::Empty,
  };
  let j = serde_json::to_string(&opened)?;
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

fn handle_command(command: &Command) {
  match command.cmd {
    UserGetGlobalSettings => info!("userGetGlobal"),
    // ChannelJoin => info!("channelJoin"),
    // SiteInfo => info!("siteInfo"),
    _ => error!("Unknown command: '{:?}'", command.cmd),
  }
}

