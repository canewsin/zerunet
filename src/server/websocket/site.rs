use actix_web_actors::ws::WebsocketContext;
use super::ZeruWebsocket;
use super::request::Command;
use super::response::Message;
use super::error::Error;
use log::*;
use futures::executor::block_on;

pub fn handle_site_info(
	socket: &ZeruWebsocket,
	ctx: &mut WebsocketContext<ZeruWebsocket>,
	command: &Command,
) -> Result<Message, Error> {
	warn!("Handling SiteInfo request with dummy response");
	let site_info_req = crate::site::SiteInfoRequest {};
	let result = block_on(socket.site_addr.send(site_info_req));
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

pub fn handle_site_list(
	socket: &ZeruWebsocket,
	ctx: &mut WebsocketContext<ZeruWebsocket>,
	command: &Command,
) -> Result<Message, Error> {
	info!("Handling SiteList");
	// TODO: actually return list of sites
	let sites = block_on(
		socket
			.site_manager
			.send(crate::site::site_manager::SiteInfoListRequest {}),
	)
	.unwrap()
	.unwrap();
	command.respond(sites)
}
