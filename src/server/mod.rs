use actix_web::{
  App, HttpServer, Result, HttpResponse, HttpRequest,
  Responder,
  web::{Data, Query, get},
};
use log::*;
use std::sync::{Mutex};

mod wrapper;
mod websocket;
mod site;

use wrapper::{serve_uimedia, serve_wrapper};
use websocket::serve_websocket;
use site::serve_file;
use std::collections::{HashSet, HashMap};

const SERVER_URL: &str = &"127.0.0.1";
const SERVER_PORT: usize = 42110;

pub struct ZeroServer {
  wrapper_nonces: Mutex<HashSet<String>>,
}

fn index() -> Result<String> {
  Ok(format!("Welcome!"))
}

pub fn run() {
  let shared_data = Data::new(ZeroServer{
    wrapper_nonces: Mutex::new(HashSet::new()),
  });

  HttpServer::new(move || {
    App::new()
      .register_data(shared_data.clone())
      .route("/", get().to(index))
      .route("/ZeroNet-Internal/Websocket", get().to(serve_websocket))
      // Debug
      // Console
      // Benchmark
      // About
      .route("/uimedia/{inner_path:.*}", get().to(serve_uimedia))
      .route("/{address:1.+}", get().to(serve_site))
      .route("/{address:1.+}/{inner_path:.*}", get().to(serve_site))
      .route("/{inner_path}", get().to(serve_uimedia))
      // .route("/{address}/{file_url:.*}", get().to(serve_site))
      // .route("/{address}", get().to(serve_site))
      // .route("/{address}/{inner_path:.*}", get().to(serve_site))
    })
    .bind(format!("{}:{}", SERVER_URL, SERVER_PORT))
    .unwrap()
    .run()
    .unwrap();
}

fn serve_site(
  req: HttpRequest,
  query: Query<HashMap<String,String>>,
  data: Data<ZeroServer>)
  -> HttpResponse
{
  let mut wrapper = true;
  let address = req.match_info().query("address");
  let inner_path = req.match_info().query("inner_path");
  if inner_path == "favicon.ico" {
    return serve_uimedia(req);
  } else if inner_path.len() > 0 {
    wrapper = false;
  } else {
    let mut wrapper_nonces = data.wrapper_nonces.lock().unwrap();
    let wrapper_nonce = query.get("wrapper_nonce");
    if wrapper_nonce.is_some() && wrapper_nonces.contains(wrapper_nonce.unwrap()) {
      wrapper_nonces.remove(wrapper_nonce.unwrap());
      wrapper = false;
    } else if wrapper_nonce.is_some() {
      warn!("Nonce {:?} not found!", wrapper_nonce);
    }
  } // wrapper_nonces lock released here

  if wrapper {
    trace!("No valid nonce provided, serving wrapper for {}", address);
    return serve_wrapper(req, data)
  }
  match serve_file(&req, data) {
    Ok(res) => match res.respond_to(&req) {
        Ok(r) => return r,
        Err(_) => HttpResponse::BadRequest().finish(),
      },
    Err(_) => HttpResponse::BadRequest().finish(),
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
