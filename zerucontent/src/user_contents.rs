use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::collections::BTreeMap;
use std::default::Default;

#[derive(Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct UserContents {
	archived: BTreeMap<String, usize>,
	archived_before: usize,
	cert_signers: BTreeMap<String, Vec<String>>,
	cert_signers_pattern: String,
	permission_rules: BTreeMap<String, PermissionRules>,
	permissions: BTreeMap<String, PermissionRules>,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct PermissionRules {
	files_allowed: String,
	max_size: usize,
}
