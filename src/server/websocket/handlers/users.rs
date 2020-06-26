use super::super::error::Error;
use super::super::request::Command;
use super::super::response::Message;
use super::super::ZeruWebsocket;
use actix_web_actors::ws::WebsocketContext;
use futures::executor::block_on;
use log::*;

pub fn handle_user_get_settings(
	ws: &ZeruWebsocket,
	ctx: &mut WebsocketContext<ZeruWebsocket>,
	command: &Command,
) -> Result<Message, Error> {
	warn!("Handling UserGetSettings with dummy response");
	// TODO: actually return user settings

	let mut map = serde_json::Map::new();
	map.insert(String::from("sites_section_hide"), serde_json::Value::Null);
	command.respond(serde_json::Value::Object(map))
}

pub fn handle_user_get_global_settings(
	ws: &ZeruWebsocket,
	ctx: &mut WebsocketContext<ZeruWebsocket>,
	command: &Command,
) -> Result<Message, Error> {
	// TODO: send message to user_manager_addr asking for user
	// then forward settings to websocket
	let user = block_on(
		ws.user_manager
			.send(crate::user::user_manager::UserRequest {
				address: String::new(),
			}),
	);
	match user {
		Ok(Some(u)) => command.respond(serde_json::to_string(&u.settings)?),
		_ => return Err(Error {}),
	}
}
