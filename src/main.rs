mod content;
mod content_manager;
mod crypto;
mod environment;
mod error;
mod influx_logger;
mod peer;
mod server;
mod site;
mod upnp;
mod util;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use crypto::zerusign;
use rand;
use serde_json;
use std::path::PathBuf;

use log::*;
use pretty_env_logger;
use peer::local_discovery::start_local_discovery;

// curl "http://localhost:9999/api/v2/write?org=zerunet&bucket=zeronet&precision=s" \                        Fri 27 Sep 2019 23:57:24 CEST
//      --header "Authorization: Token hgt8JHm1c6c9_rD_lumpXNEf1qCjVqyT13AOSzrlbZfhlKEIc5MaMfKgZq8H4w1wHDCsFICF-UGEI3Zok5OiMg==" \
//      --data-raw "mem,host=host1 used_percent=27"


fn main() {
	let data_path = PathBuf::from("/home/crolsi/Programs/ZeroNet/data/");
	// influx_logger::init();
	pretty_env_logger::init_timed();

	environment::get_env();

	if false {
		let punch = upnp::UPnBrunch::new()
			.unwrap()
			.retries(3)
			.add_protocol(upnp::Protocol::TCP);
		if punch.open_port(15443, "ZeroNet").is_ok() {
			info!("Port opened!");
		} else {
			error!("Failed to open port");
		}
		if punch.close_port(15443).is_ok() {
			info!("Port closed!");
		} else {
			error!("Failed to close port");
		}
	}

	let res = start_local_discovery();
	info!("{:?}", res);

	info!("Starting zerunet server.");
	server::run();

	let content_path = Path::new("test/content.json");
	let file = match File::open(content_path) {
		Ok(f) => f,
		Err(_) => return,
	};

	let test_content: content::Content = match serde_json::from_reader(BufReader::new(file)) {
		Ok(c) => c,
		Err(error) => {
			println!("error {:?}", error);
			return;
		}
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

	let test_msg = String::from("testmessage");

	let key = String::from("1JUDmCT4UCSdnPsJAHBoXNkDS61Y31Ue52");

	let value = match test_content.signs.get(&key) {
		Some(v) => v,
		None => return,
	};

	match zerusign::verify(
		test_content.dump().unwrap().to_string(),
		key,
		String::from(value),
	) {
		Ok(_) => info!("Signature valid!"),
		Err(_) => error!("Signature mismatch!"),
	}

	let secp = secp256k1::Secp256k1::new();
	let mut rng = rand::rngs::OsRng::new().expect("OsRng");
	let (privkey, pubkey) = secp.generate_keypair(&mut rng);
	info!("{:?}", privkey);
	info!("{:?}", pubkey);
}
