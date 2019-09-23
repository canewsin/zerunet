use serde::{ Serialize, Deserialize };
use std::default::Default;
use std::cmp::PartialEq;

#[derive(Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct File {
  sha512: String,
  size: u64,
}
