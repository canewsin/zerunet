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
use log::*;

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
	pub files: BTreeMap<String, File>,
	#[serde(skip_serializing_if = "is_default")]
	pub files_optional: BTreeMap<String, File>,
	modified: Option<serde_json::Number>, // TODO: this is not actually an option
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
	ignore: Option<String>,
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
	#[serde(skip_serializing_if = "is_default")]
	settings: BTreeMap<String, serde_json::Value>,

	#[serde(flatten)]
	other: BTreeMap<String, Value>,
}

impl Content {
	pub fn from_buf(buf: serde_bytes::ByteBuf) -> Result<Content, ()> {
		let content = match serde_json::from_slice(&buf) {
			Ok(c) => c,
			Err(_) => return Err(()),
		};
		return Ok(content)
	}
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
	pub fn verify(&self, key: String) -> bool {
		let content = self.cleared();
		trace!("{:?}", content.dump());
		let signature = match self.signs.get(&key) {
			Some(v) => v,
			None => {
				error!("Could not find signature for {}", &key);
				return false
			},
		};
		let result = crate::crypto::zerusign::verify(
			content.dump().unwrap().to_string(),
			key.clone(),
			signature.to_string(),
		);
		if !result.is_ok() {
			error!("Signature verification failed key {}, sig {}: {:?}", &key, &signature, &result);
		}
		return result.is_ok()
	}
	pub fn sign(&self, privkey: String) -> String {
		let content = self.cleared();
		let result = crate::crypto::zerusign::sign(
			content.dump().unwrap().to_string(),
			privkey,
		).unwrap();
		return result
	}
	pub fn get_file(&self, inner_path: &str) -> Option<File> {
		if let Some(f) = self.files.get(inner_path) {
			return Some(f.clone());
		} else  if let Some(f) = self.files_optional.get(inner_path) {
			return Some(f.clone());
		}
		return None
	}
}

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod tests {
	use super::*;

	#[test]
	fn test_verification() {
		let content: Content = serde_json::from_str("
		{
			\"address\": \"1JUDmCT4UCSdnPsJAHBoXNkDS61Y31Ue52\",
			\"address_index\": 36579623,
			\"background-color\": \"white\",
			\"cloneable\": true,
			\"cloned_from\": \"1RedkCkVaXuVXrqCMpoXQS29bwaqsuFdL\",
			\"description\": \"Home of the bots\",
			\"files\": {
			 \"data-default/users/content.json-default\": {
				\"sha512\": \"4e37699bd5336b9c33ce86a3eb73b82e87460535793401874a653afeddefee59\",
				\"size\": 735
			 },
			 \"index.html\": {
				\"sha512\": \"087c6ae46aacc5661f7da99ce10dacc0428dbd48aa7bbdc1df9c2da6e81b1d93\",
				\"size\": 466
			 }
			},
			\"ignore\": \"((js|css)/(?!all.(js|css))|data/.*db|data/users/.*/.*)\",
			\"includes\": {
			 \"data/users/content.json\": {
				\"signers\": [],
				\"signers_required\": 1
			 }
			},
			\"inner_path\": \"content.json\",
			\"merged_type\": \"ZeroMe\",
			\"modified\": 1471656205.079839,
			\"postmessage_nonce_security\": true,
			\"sign\": [
			 60601328857260736769667767617236149396007806053808183569130735997086722937268,
			 43661716327244911082383801335054839207111588960552431293232589470692186442781
			],
			\"signers_sign\": \"HEMH4/a7LXic4PYgMj/4toV5jI5z+SX6Bnmo3mP0HoyIGy6e7rUbilJYAH3MrgCT/IXzIn7cnIlhL8VARh7CeUg=\",
			\"signs\": {
			 \"1JUDmCT4UCSdnPsJAHBoXNkDS61Y31Ue52\": \"G5qMkd9+n0FMLm2KA4FAN3cz/vaGY/oSYd2k/edx4C+TIv76NQI37NsjXVWtkckMoxvp6rhW8PHZy9Q1MNtmIAM=\"
			},
			\"signs_required\": 1,
			\"title\": \"Bot Hub\",
			\"zeronet_version\": \"0.4.0\"
		 }").unwrap();
		let key = String::from("1JUDmCT4UCSdnPsJAHBoXNkDS61Y31Ue52");
		let result = content.verify(key);
		assert_eq!(result, true)
	}
}