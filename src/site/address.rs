use crate::error::Error;
use log::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};
use std::cmp::{Eq, PartialEq};
use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

// TODO: remove default
#[derive(Hash, PartialEq, Eq, Debug, Clone, Default)]
pub struct Address {
	address: String,
}

impl Address {
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

impl Serialize for Address {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.address)
	}
}

// impl<'de> Deserialize<'de> for Address {
// 	fn deserialize<D>(deserializer: D) -> Result<Address, D::Error>
// 	where
// 		D: Deserializer<'de>,
// 	{
// 		serde::deserialize_str(deserializer).map(|s| Address::from_str(s).unwrap())
// 	}
// }

impl FromStr for Address {
	type Err = Error;

	fn from_str(string: &str) -> Result<Address, Error> {
		let s = String::from(string);
		if string == "Test" {
			return Ok(Address {
				address: String::from(string),
			});
		}
		if s.len() > 34 || s.len() < 33 || !s.starts_with('1') {
			error!(
				"Length should be 34 or 33, was {}, and start with a '1'.",
				string.len(),
			);
			return Err(Error::FileNotFound);
		}
		Ok(Address {
			address: String::from(string),
		})
	}
}

impl Into<String> for Address {
	fn into(self) -> String {
		self.address.clone()
	}
}

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod tests {
	use super::*;

	#[test]
	fn test_creation() {
		let result = Address::from_str("1HeLLo4uzjaLetFx6NH3PMwFP3qbRbTf3D");
		assert_eq!(result.is_ok(), true);
	}

	#[test]
	fn test_serialization() {
		let result = Address::from_str("1HeLLo4uzjaLetFx6NH3PMwFP3qbRbTf3D");
		assert_eq!(result.is_ok(), true, "Encountered error: {:?}", result);
		let address = result.unwrap();
		let result = serde_json::to_string(&address);
		assert_eq!(result.is_ok(), true);
		assert_eq!(result.unwrap(), "\"1HeLLo4uzjaLetFx6NH3PMwFP3qbRbTf3D\"");
	}

	// 	#[test]
	// 	fn test_deserialization() {
	// 		let result = serde_json::from_str("\"1HeLLo4uzjaLetFx6NH3PMwFP3qbRbTf3D\"");
	// 		assert_eq!(result.is_ok(), true, "Encountered error: {:?}", result);
	// 		let address: Address = result.unwrap();
	// 		assert_eq!(address, Address{ address: String::from("1HeLLo4uzjaLetFx6NH3PMwFP3qbRbTf3D") });
	// 	}
}
