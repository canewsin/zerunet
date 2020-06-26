pub mod user_manager;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
	#[serde(default)]
	master_address: String,
	master_seed: String,
	sites: serde_json::Value,
	certs: serde_json::Value,
	pub settings: HashMap<String, serde_json::Value>,
}

impl User {
	/// Creates a new user with a new seed and address pair
	fn new() -> User {
		let (master_seed, master_address) = zerucrypt::create();
		User {
			master_seed,
			master_address,
			sites: serde_json::Value::Null,
			certs: serde_json::Value::Null,
			settings: HashMap::new(),
		}
	}

	/// Creates a new user from a seed
	fn from_seed(master_seed: String) -> User {
		let master_address = String::new();
		User {
			master_seed,
			master_address,
			sites: serde_json::Value::Null,
			certs: serde_json::Value::Null,
			settings: HashMap::new(),
		}
	}

	fn get_address_auth_index() {}

	fn generate_auth_address() {}

	/// Get user site data
	///
	/// Return: {"auth_address": "1AddR", "auth_privatekey": "xxx"}
	fn get_site_data() {}

	fn delete_site_data() {}

	fn set_site_settings() {}

	/// Get data for a new, unique site
	///
	/// Return: [site_address, bip32_index, {"auth_address": "1AddR", "auth_privatekey": "xxx", "privatekey": "xxx"}]
	fn get_new_site_data() {}
	/// Get BIP32 address from site address
	///
	/// Return: BIP32 auth address
	fn get_auth_address() {}

	fn get_auth_privatekey() {}

	/// Add cert for the user
	fn add_cert() {}

	/// Remove cert from user
	fn delete_cert() {}

	/// Set active cert for a site
	fn set_cert() {}

	/// Get cert for the site address
	///
	/// Return: { "auth_address": "1AddR", "auth_privatekey": "xxx", "auth_type": "web", "auth_user_name": "nofish", "cert_sign": "xxx"} or None
	fn get_cert() {}

	/// Get cert user name for the site address
	///
	/// Return user@certprovider.bit or None
	fn get_cert_user_id() {}
}
