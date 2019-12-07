use super::address::Address;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SiteInfo {
	pub tasks: usize,
	pub size_limit: usize,
	pub address: Address,
	pub next_size_limit: usize,
	pub auth_address: String,
	pub auth_key_sha512: String,
	pub peers: usize,
	pub auth_key: String,
	// settings: SiteSettings,
	// bad_files: usize,
	// workers: usize,
	// content: SiteContentSummary,
	// cert_user_id: String,
	// started_task_num: usize,
	// content_updated: f64,
}

#[derive(Serialize, Deserialize)]
pub struct SiteSettings {
	peers: usize,
	serving: bool,
	modified: f64,
	own: bool,
	permissions: Vec<String>,
	size: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SiteContentSummary {
	files: usize,
	description: String,
	title: String,
	signs_required: usize,
	modified: f64,
	ignore: String,
	signers_sign: Option<String>,
	address: Address,
	zeronet_version: String,
	includes: usize,
}
