use serde::{Serialize, Deserialize};
use std::hash::Hash;
use std::cmp::{PartialEq, Eq};
use crate::error::Error;
use std::fmt::Display;
use log::*;

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Debug, Clone)]
pub struct Address {
  address: String
}

impl Address {
  pub fn from_str(string: &str) -> Result<Address, Error> {
    let s = String::from(string);
    if s.len() != 34 || !s.starts_with('1') {
      error!("Length should be 34, was {}, and start with a '1'.", string.len(),);
      return Err(Error::FileNotFound);
    }
    Ok(Address {
      address: String::from(string)
    })
  }
  // digest of Sha256 hash of ASCII encoding
  pub fn get_address_hash(&self) -> String {
    self.address.clone()
  }
  // digest of Sha1 hash of ACII encoding
  pub fn get_address_sha1(&self) -> String {
    self.address.clone()
  }
  // first 6 and last 4 characters of address
  pub fn get_address_short(&self) -> String {
    let l = self.address.len();
    // TODO: remove unwraps
    let f = self.address.get(0..6).unwrap();
    let b = self.address.get(l-5..l).unwrap();
    format!("{}...{}", f, b)
  }
}

impl Display for Address {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.address)
  }
}
