use actix_web_actors::ws::WebsocketContext;
use super::super::ZeruWebsocket;
use super::super::request::Command;
use super::super::response::Message;
use super::super::error::Error;
use log::*;
use futures::executor::block_on;

pub fn handle_site_info(
	ws: &ZeruWebsocket,
	ctx: &mut WebsocketContext<ZeruWebsocket>,
	command: &Command,
) -> Result<Message, Error> {
	warn!("Handling SiteInfo request with dummy response");
	let site_info_req = crate::site::SiteInfoRequest {};
	let result = block_on(ws.site_addr.send(site_info_req));
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
	ws: &ZeruWebsocket,
	ctx: &mut WebsocketContext<ZeruWebsocket>,
	command: &Command,
) -> Result<Message, Error> {
	info!("Handling SiteList");
	// TODO: actually return list of sites
	let sites = block_on(
		ws
			.site_manager
			.send(crate::site::site_manager::SiteInfoListRequest {}),
	)
	.unwrap()
	.unwrap();
	command.respond(sites)
}

pub fn handle_optional_limit_stats(
	ws: &ZeruWebsocket,
	ctx: &mut WebsocketContext<ZeruWebsocket>,
	command: &Command,
) -> Result<Message, Error> {
	// TODO: replace dummy response with actual response
	warn!("Handling OptionalLimitStats with dummy response");
	let limit_stats = crate::optional_files::OptionalLimitStats {
		limit: String::from("10%"),
		used: 1000000,
		free: 4000000,
	};
	command.respond(limit_stats)
}

pub fn handle_channel_join(
	ws: &ZeruWebsocket,
	ctx: &mut WebsocketContext<ZeruWebsocket>,
	command: &Command,
) -> Result<Message, Error> {
	warn!("Handling ChannelJoin request using dummy response");
	command.respond(String::from("ok"))
}

pub fn handle_channel_join_all_site(
	ws: &ZeruWebsocket,
	ctx: &mut WebsocketContext<ZeruWebsocket>,
	command: &Command,
) -> Result<Message, Error> {
	warn!("Handling ChannelJoinAllsite request using dummy response");
	command.respond(String::from("ok"))
}
