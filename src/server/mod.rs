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

const SERVER_URL: &str = &"127.0.0.1:42110";

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
      .route("/uimedia/{file_url:.*}", get().to(serve_uimedia))
      .route("/{address}", get().to(serve_site))
      .route("/{address}/{inner_path:.*}", get().to(serve_site))
      // .route("/{address}/{file_url:.*}", get().to(serve_site))
      // .route("/{address}", get().to(serve_site))
      // .route("/{address}/{inner_path:.*}", get().to(serve_site))
    })
    .bind(SERVER_URL)
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
  // info!("Req: {:?}", req);
  if req.match_info().query("inner_path").len() > 0 {
    info!("Received file request {}", req.match_info().query("inner_path"));
    wrapper = false;
  } else {
    let mut wrapper_nonces = data.wrapper_nonces.lock().unwrap();
    let wrapper_nonce = query.get("wrapper_nonce");
    if wrapper_nonce.is_some() && wrapper_nonces.contains(wrapper_nonce.unwrap()) {
      info!("Nonce matches!");
      wrapper_nonces.remove(wrapper_nonce.unwrap());
      wrapper = false;
    } else {
      warn!("Nonce {:?} not found!", wrapper_nonce);
    }
  }

  if wrapper {
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
