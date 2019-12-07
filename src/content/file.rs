use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::default::Default;

#[derive(Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct File {
	sha512: String,
	size: u64,
}
