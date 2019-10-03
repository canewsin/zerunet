mod content;
mod content_manager;
mod zerusign;
mod server;
mod influx_logger;

use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;

use serde_json;
use bitcoin::util::misc::signed_msg_hash;
use ripemd160::{ Ripemd160, Digest };
use base64::{ encode, decode };
use secp256k1::{ Secp256k1, Message, recovery };
use rand::rngs::{ OsRng };
use sha2::{Sha256};
use std::path::PathBuf;

use sha2::digest::FixedOutput;
use reqwest;

use signatory::{
  ecdsa::{
    curve::nistp256::{ self, FixedSignature },
    PublicKey,
  },
  // encoding::FRomPkcs8,
  generic_array::GenericArray,
  signature::{ Signature, Signer as _, Verifier as _ },
};
use signatory_ring::ecdsa::p256::{ Signer, Verifier };
use pretty_env_logger;
#[macro_use]
use log::*;

// curl "http://localhost:9999/api/v2/write?org=zerunet&bucket=zeronet&precision=s" \                        Fri 27 Sep 2019 23:57:24 CEST
//      --header "Authorization: Token hgt8JHm1c6c9_rD_lumpXNEf1qCjVqyT13AOSzrlbZfhlKEIc5MaMfKgZq8H4w1wHDCsFICF-UGEI3Zok5OiMg==" \
//      --data-raw "mem,host=host1 used_percent=27"

fn main() {
    let data_path = PathBuf::from("/home/crolsi/Programs/ZeroNet/data/");
    pretty_env_logger::init();
    // influx_logger::init();

    info!("starting zerunet");

    let zero_server = server::ZeroServer::new();

    let content_path = Path::new("test/content.json");
    let file = match File::open(content_path) {
      Ok(f) => f,
      Err(_) => return,
    };

    let test_content: content::Content = match serde_json::from_reader(BufReader::new(file)) {
      Ok(c) => c,
      Err(error) => { println!("error {:?}", error); return },
    };

    let test_content2 = test_content.cleared();

    let new_content_path = Path::new("test/content-new.json");
    let mut new_file = match File::create(new_content_path) {
      Ok(f) => f,
      Err(_) => return,
    };

    let string = match serde_json::to_string(&test_content2) {
      Ok(s) => s,
      Err(_) => return,
    };

    new_file.write_all(&test_content.dump().unwrap().to_string().as_bytes());

    // let secp = secp256k1::Secp256k1::new();
    // let mut rng: OsRng = OsRng::new().expect("OsRng");
    // let (privkey, pubkey) = secp.generate_keypair(&mut rng);

    let test_msg = String::from("testmessage");
    // let test_sig = secp.sign(test_msg, privkey);

    let key = String::from("1JUDmCT4UCSdnPsJAHBoXNkDS61Y31Ue52");

    let value = match test_content.signs.get(&key) {
      Some(v) => { v },
      None => { return },
    };

    match zerusign::verify(test_content.dump().unwrap().to_string(), key, String::from(value)) {
      Ok(_) => info!("Signature valid!"),
      Err(_) => error!("Signature mismatch!"),
    }
    match zerusign::verify(
      String::from("Testmessage"),
      String::from("1HZwkjkeaoZfTSaJxDw6aKkxp45agDiEzN"),
      String::from("G+Hnv6dXxOAmtCj8MwQrOh5m5bV9QrmQi7DSGKiRGm9TWqWP3c5uYxUI/C/c+m9+LtYO26GbVnvuwu7hVPpUdow=")
    ) {
      Ok(_) => info!("Signature valid!"),
      Err(_) => error!("Signature mismatch!"),
    }
}
