mod address;
mod site_info;

use address::Address;
use actix::prelude::*;
use actix;
use std::collections::HashMap;

pub struct SiteManager {
  sites: HashMap<Address, Site>,
}

pub struct Site {
  address: Address,
}

impl Site {
  pub fn new() -> Site {
    Site {
      address: Address::from_str("1ETC")
    }
  }
}

impl Actor for Site {
  type Context = Context<Self>;
}

#[derive(Debug)]
pub struct FileRequest(pub String);

impl Message for FileRequest {
  type Result = Result<bool, ()>;
}

impl Handler<FileRequest> for Site {
  type Result = Result<bool, ()>;

  fn handle(&mut self, msg: FileRequest, ctx: &mut Context<Self>) -> Self::Result {
    println!("filerequest received {:?}", msg);

    Ok(true)
  }
}
