use actix::{Actor, StreamHandler};
use actix_web::{HttpRequest, Result, Error, HttpResponse};
use actix_web::web::Payload;
use actix_web_actors::ws;
use log::*;
use futures::Future;
use serde::{Serialize, Deserialize};
use crate::site;

pub fn serve_websocket(req: HttpRequest, stream: Payload) -> Result<HttpResponse, Error> {
  let site = site::Site::new();
  let addr: actix::Addr<site::Site> = site.start();

  ws::start(ZeruWebsocket {
    site_addr: addr,
  }, &req, stream)
}

struct ZeruWebsocket {
  site_addr: actix::Addr<site::Site>,
}

impl Actor for ZeruWebsocket {
  type Context = ws::WebsocketContext<Self>;
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
              Ok(result) => println!("got result {:?}", result),
              Err(err) => println!("got err {:?}", err),
            }
          }).map_err(|err| {
            println!("Actor is probably dead {:?}", err);
          })
        );

        let command: Command = match serde_json::from_str(&text) {
          Ok(c) => c,
          Err(_) => {error!("unknown command {:?}", text); return},
        };
        handle_command(command);
      },
      ws::Message::Binary(bin) => ctx.binary(bin),
      _ => (),
    }
  }
}

use CommandType::*;

fn handle_command(command: Command) {
  match command.cmd {
    UserGetGlobalSettings => info!("userGetGlobal"),
    // ChannelJoin => info!("channelJoin"),
    // SiteInfo => info!("siteInfo"),
    _ => error!("Unknown command: '{:?}'", command.cmd),
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum CommandType {
  AnnouncerInfo,
  CertAdd,
  CertSelect,
  ChannelJoin,
  DbQuery,
  DirList,
  FileDelete,
  FileGet,
  FileList,
  FileNeed,
  FileQuery,
  FileRules,
  FileWrite,
  Ping,
  ServerInfo,
  SiteInfo,
  SitePublish,
  SiteReload,
  SiteSign,
  SiteUpdate,
  UserGetGlobalSettings,
  UserSetGlobalSettings,
  // Bigfile
  BigFileUploadInit,
  // Cors
  CorsPermission,
  // Multiuser
  UserLoginForm,
  UserShowMasterSeed,
  // CryptMessage
  UserPublickey,
  EciesEncrypt,
  EciesDecrypt,
  AesEncrypt,
  AesDecrypt,
  // Newsfeed
  FeedFollow,
  FeedListFollow,
  FeedQuery,
  // MergerSite
  MergerSiteAdd,
  MergerSiteDelete,
  MergerSiteList,
  // Mute
  MuteAdd,
  MuteRemove,
  MuteList,
  // OptionalManager
  OptionalFileList,
  OptionalFileInfo,
  OptionalFilePin,
  OptionalFileUnpin,
  OptionalFileDelete,
  OptionalLimitStats,
  OptionalLimitSet,
  OptionalHelpList,
  OptionalHelp,
  OptionalHelpRemove,
  OptionalHelpAll,
  // Admin commands
  As,
  CertList,
  CertSet,
  ChannelJoinAllsite,
  ConfigSet,
  ServerPortcheck,
  ServerShutdown,
  ServerUpdate,
  SiteClone,
  SiteList,
  SitePause,
  SiteResume,
}

#[derive(Serialize, Deserialize, Debug)]
struct Command {
  cmd: CommandType,
  params: serde_json::Value,
  id: isize,
}
