use serde::{ Serialize, Deserialize };
use std::default::Default;
use std::cmp::PartialEq;
use super::is_default;

#[derive(Serialize, Deserialize, Default, PartialEq, Clone)]
#[serde(default)]
pub struct Include {
  signers: Vec<String>,
  #[serde(skip_serializing_if = "is_default")]
  signers_required: u64,
  #[serde(skip_serializing_if = "is_default")]
  files_allowed: String,
  #[serde(skip_serializing_if = "is_default")]
  includes_allowed: bool,
  #[serde(skip_serializing_if = "is_default")]
  max_size: u64,
}