use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Address {
  address: String
}

impl Address {
  pub fn from_str(string: &str) -> Address {
    Address {
      address: String::from(string)
    }
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
    format!("{}...{}", self.address, self.address) // TODO: select 6, -4
  }
}
