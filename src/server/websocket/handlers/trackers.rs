use actix_web_actors::ws::WebsocketContext;
use super::super::ZeruWebsocket;
use super::super::request::Command;
use super::super::response::Message;
use super::super::error::Error;
use log::*;
use std::collections::HashMap;

pub fn handle_announcer_stats(
	ws: &ZeruWebsocket,
	ctx: &mut WebsocketContext<ZeruWebsocket>,
	command: &Command,
) -> Result<Message, Error> {
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