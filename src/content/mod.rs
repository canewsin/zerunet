mod file;
mod include;
mod user_contents;
mod zeroformatter;

use json_filter_sorted::sort::sort_json;

use file::File;
use include::Include;
use user_contents::UserContents;

use crate::util::is_default;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::default::Default;

// use json_filter_sorted::sort::sort_json;

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct Content {
	address: String,

	#[serde(skip_serializing_if = "is_default")]
	address_index: usize,
	#[serde(skip_serializing_if = "is_default")]
	cloned_from: String,
	#[serde(skip_serializing_if = "is_default")]
	clone_root: String,
	files: BTreeMap<String, File>,
	#[serde(skip_serializing_if = "is_default")]
	files_optional: BTreeMap<String, File>,
	modified: f64,
	#[serde(skip_serializing_if = "is_default")]
	sign: Vec<f64>, // DEPRECATED
	#[serde(skip_serializing_if = "is_default")]
	signers_sign: String,
	#[serde(skip_serializing_if = "is_default")]
	pub signs: BTreeMap<String, String>,
	zeronet_version: String,

	#[serde(rename = "background-color")]
	#[serde(skip_serializing_if = "is_default")]
	background_color: String,
	#[serde(skip_serializing_if = "is_default")]
	cloneable: bool,
	#[serde(skip_serializing_if = "is_default")]
	description: String,
	#[serde(skip_serializing_if = "is_default")]
	domain: String,
	#[serde(skip_serializing_if = "is_default")]
	ignore: String,
	#[serde(skip_serializing_if = "is_default")]
	includes: BTreeMap<String, Include>,
	#[serde(skip_serializing_if = "is_default")]
	merged_type: String,
	#[serde(skip_serializing_if = "is_default")]
	optional: String,
	#[serde(skip_serializing_if = "is_default")]
	signs_required: usize,
	#[serde(skip_serializing_if = "is_default")]
	title: String,
	#[serde(skip_serializing_if = "is_default")]
	translate: Vec<String>,
	#[serde(skip_serializing_if = "is_default")]
	favicon: String,
	#[serde(skip_serializing_if = "is_default")]
	user_contents: UserContents,
	#[serde(skip_serializing_if = "is_default")]
	viewport: String,

	#[serde(flatten)]
	other: BTreeMap<String, Value>,
}

impl Content {
	pub fn cleared(&self) -> Content {
		let mut new_content = self.clone();
		new_content.signs = BTreeMap::new();
		new_content.sign = vec![];
		new_content
	}
	pub fn dump(&self) -> Result<String, serde_json::error::Error> {
		zeroformatter::to_string_zero(
			&sort_json(json!(self.cleared()))
				.unwrap()
				.as_object()
				.map(|x| x.to_owned())
				.unwrap(),
		)
	}
}
