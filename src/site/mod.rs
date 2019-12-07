pub mod address;
mod site_info;
pub mod site_manager;

use actix;
use actix::prelude::*;
use address::Address;
use site_info::SiteInfo;

#[derive(Debug)]
pub struct Site {
	address: Address,
}

impl Site {
	pub fn new() -> Site {
		Site {
			address: Address::from_str("1HeLLo4uzjaLetFx6NH3PMwFP3qbRbTf3D").unwrap(),
		}
	}
}

impl Actor for Site {
	type Context = Context<Self>;
}

pub struct SiteInfoRequest {}

impl Message for SiteInfoRequest {
	type Result = Result<SiteInfo, ()>;
}

impl Handler<SiteInfoRequest> for Site {
	type Result = Result<SiteInfo, ()>;

	fn handle(&mut self, msg: SiteInfoRequest, ctx: &mut Context<Self>) -> Self::Result {
		Ok(SiteInfo {
			tasks: 1,
			size_limit: 1,
			address: self.address.clone(),
			next_size_limit: 2,
			auth_address: String::from("test"),
			auth_key_sha512: String::from("test"),
			peers: 1,
			auth_key: String::from("test"),
		})
	}
}
