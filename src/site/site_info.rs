use super::address::Address;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SiteSettings {
	pub peers: usize,
	pub serving: bool,
	pub modified: f64,
	pub own: bool,
	pub permissions: Vec<String>,
	pub size: usize,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SiteContentSummary {
	address: String,
	address_index: usize,
	// background_color:
	// clone_root
	// cloned_from
	description: String,
	files: usize,
	ignore: Option<String>,
	inner_path: String,
	title: String,
	signs_required: usize,
	modified: f64,
	signers_sign: Option<String>,
	translate: Vec<String>,
	zeronet_version: String,
	files_optional: usize,
	includes: usize,
}

impl SiteContentSummary {
	pub fn from_content(content: &crate::content::Content) -> SiteContentSummary {
		SiteContentSummary {
			address: content.address.clone(),
			address_index: 0,
			description: content.description.clone(),
			files: content.files.len(),
			files_optional: content.files_optional.len(),
			ignore: content.ignore.clone(),
			includes: content.includes.len(),
			inner_path: String::from("content.json"),
			modified: 0.0, // TODO: replace hardcoded
			signs_required: content.signs_required,
			signers_sign: None,
			title: content.title.clone(),
			translate: vec![],
			zeronet_version: content.zeronet_version.clone(),
		}
	}
}
