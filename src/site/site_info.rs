use super::address::Address;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SiteInfo {
	tasks: usize,
	size_limit: usize,
	address: Address,
	next_size_limit: usize,
	auth_address: String,
	auth_key_sha512: String,
	peers: usize,
	auth_key: String,
	settings: SiteSettings,
	bad_files: usize,
	workers: usize,
	content: SiteContentSummary,
	cert_user_id: String,
	started_task_num: usize,
	content_updated: f64,
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