use super::address::Address;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SiteInfo {
	pub tasks: usize,
	// Allowed size in MB
	pub size_limit: usize,
	pub address: String,
	pub next_size_limit: usize,
	pub auth_address: String,
	pub auth_key_sha512: String,
	pub peers: usize,
	pub auth_key: String,
	pub settings: SiteSettings,
	pub bad_files: usize,
	pub workers: usize,
	pub content: SiteContentSummary,
	// cert_user_id: String,
	pub started_task_num: usize,
	pub content_updated: f64,
}

#[derive(Serialize, Deserialize)]
pub struct SiteSettings {
	pub peers: usize,
	pub serving: bool,
	pub modified: f64,
	pub own: bool,
	pub permissions: Vec<String>,
	pub size: usize,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SiteContentSummary {
	files: usize,
	description: String,
	title: String,
	signs_required: usize,
	modified: f64,
	ignore: String,
	signers_sign: Option<String>,
	address: String,
	zeronet_version: String,
	includes: usize,
}
