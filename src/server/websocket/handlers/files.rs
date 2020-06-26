use super::super::error::Error;
use super::super::request::Command;
use super::super::response::Message;
use super::super::ZeruWebsocket;
use actix_web_actors::ws::WebsocketContext;
use futures::executor::block_on;
use log::*;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn handle_file_get(
	ws: &ZeruWebsocket,
	ctx: &mut WebsocketContext<ZeruWebsocket>,
	command: &Command,
) -> Result<Message, Error> {
	warn!("Handling FileGet request");
	// if required || inner_path in site.bad_files
	// if let Some(addr) = addr {
	// 	addr.send(FileNeed command);
	// }
	let msg: crate::site::FileGetRequest = match serde_json::from_value(command.params.clone()) {
		Ok(m) => m,
		Err(e) => {
			error!("{:?}", e);
			// TODO: error
			crate::site::FileGetRequest::default()
		}
	};
	let mut path = ws.data_path.clone(); // TODO: use data path env var
	path.push(Path::new(&format!(
		"{}/{}",
		ws.address.to_string(),
		msg.inner_path
	)));
	if !path.is_file() {
		block_on(ws.site_addr.send(msg));
	}
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
