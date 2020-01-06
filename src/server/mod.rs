use actix::{Actor, Addr};
use actix_web::{
	web::{get, Data, Query},
	App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use log::*;
use std::sync::{Arc, Mutex};

mod site;
pub mod websocket;
mod wrapper;

use futures::executor::block_on;
use crate::site::site_manager::SiteManager;
use site::serve_file;
use std::collections::{HashMap, HashSet};
use websocket::serve_websocket;
use wrapper::{serve_uimedia, serve_wrapper};

const SERVER_URL: &str = &"127.0.0.1";
const SERVER_PORT: usize = 42110;

pub struct ZeroServer {
	site_manager: actix::Addr<SiteManager>,
	wrapper_nonces: Arc<Mutex<HashSet<String>>>,
}

async fn index(data: Data<ZeroServer>) -> Result<String> {
	Ok(format!("Welcome!"))
}

pub async fn run(site_manager: Addr<SiteManager>) -> std::io::Result<()> {
	let nonces = Arc::new(Mutex::new(HashSet::new()));

	HttpServer::new(move || {
		let shared_data = ZeroServer {
			site_manager: site_manager.clone(),
			wrapper_nonces: nonces.clone(),
		};
		App::new()
			.data(shared_data)
			.route("/", get().to(index))
			.route("/ZeroNet-Internal/Websocket", get().to(serve_websocket))
			// Debug
			// Console
			// Benchmark
			// About
			.route("/{address:Test}/{inner_path:.*}", get().to(serve_site))
			.route("/{address:Test}", get().to(serve_site))
			.route("/uimedia/{inner_path:.*}", get().to(serve_uimedia))
			.route("/{address:1[^/]+}/{inner_path:.*}", get().to(serve_site))
			.route("/{address:1[^/]+}", get().to(serve_site))
			.route("/{inner_path}", get().to(serve_uimedia))
		// .route("/{address}/{file_url:.*}", get().to(serve_site))
		// .route("/{address}", get().to(serve_site))
		// .route("/{address}/{inner_path:.*}", get().to(serve_site))
	})
	.bind(format!("{}:{}", SERVER_URL, SERVER_PORT))
	.unwrap()
	.run().await
}

fn serve_site(
	req: HttpRequest,
	query: Query<HashMap<String, String>>,
	data: Data<ZeroServer>,
) -> HttpResponse {
	let mut wrapper = true;
	let address = req.match_info().query("address");
	let inner_path = req.match_info().query("inner_path");
	if inner_path == "favicon.ico" {
		return serve_uimedia(req);
	} else if inner_path.len() > 0 && inner_path.contains('.') && !inner_path.ends_with("/.html?/i") {
		wrapper = false;
	} else {
		let mut wrapper_nonces = data.wrapper_nonces.lock().unwrap();
		let wrapper_nonce = query.get("wrapper_nonce");
		if wrapper_nonce.is_some() && wrapper_nonces.contains(wrapper_nonce.unwrap()) {
			wrapper_nonces.remove(wrapper_nonce.unwrap());
			wrapper = false;
		} else if wrapper_nonce.is_some() {
			warn!("Nonce {:?} invalid!", wrapper_nonce);
		}
	} // wrapper_nonces lock released here

	if wrapper {
		trace!(
			"No valid nonce provided, serving wrapper for zero:://{}",
			address
		);
		return serve_wrapper(req, data);
	}
	match serve_file(&req, data) {
		Ok(res) => match block_on(res.respond_to(&req)) {
			Ok(r) => return r,
			Err(err) => {
				error!("Bad request {}", err);
				HttpResponse::BadRequest().finish()
			}
		},
		Err(err) => {
			error!("Bad request {}", err);
			HttpResponse::BadRequest().finish()
		}
	}

	// return Box::new(site::serve_file(req, data))
	// match site::serve_site(req, data) {
	//   Ok(f) => return f.into_response(req),
	//   Err(_) => return {
	//     HttpResponse::Ok()
	//       .content_type("html")
	//       .header("X-Hdr", "sample")
	//       .body("error")
	//   },
	// }
}
