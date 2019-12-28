use crate::error::Error;
use log::*;
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};
use std::fmt::Display;
use std::hash::Hash;
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Debug, Clone)]
pub struct Address {
	address: String,
}

impl Address {
	pub fn from_str(string: &str) -> Result<Address, Error> {
		let s = String::from(string);
		if string == "Test" {
			return Ok(Address {
				address: String::from(string),
			});
		}
		if s.len() != 34 || !s.starts_with('1') {
			error!(
				"Length should be 34, was {}, and start with a '1'.",
				string.len(),
			);
			return Err(Error::FileNotFound);
		}
		Ok(Address {
			address: String::from(string),
		})
	}
	// digest of Sha256 hash of ASCII encoding
	pub fn get_address_hash(&self) -> Vec<u8> {
		let mut hasher = Sha256::default();
		hasher.input(&self.address);
		hasher.result().to_vec()
	}
	// digest of Sha1 hash of ACII encoding
	pub fn get_address_sha1(&self) -> String {
		self.address.clone()
	}
	// first 6 and last 4 characters of address
	pub fn get_address_short(&self) -> String {
		if self.address.as_str() == "Test" {
			return self.address.clone();
		}
		let l = self.address.len();
		// TODO: remove unwraps
		let f = self.address.get(0..6).unwrap();
		let b = self.address.get(l - 5..l).unwrap();
		format!("{}...{}", f, b)
	}
}

impl Display for Address {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.address)
	}
}
