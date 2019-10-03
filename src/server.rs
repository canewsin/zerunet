use actix_web::{web, App, HttpServer, Result, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use log::*;
use std::fs::File;
use std::path::{ Path, PathBuf };
use std::io::Read;
use actix_files::NamedFile;

pub struct ZeroServer {
  // http_server: HttpServer,
}

#[derive(Deserialize)]
struct Info {
  username: String,
}

fn index() -> Result<String> {
  Ok(format!("Welcome!"))
}

#[derive(Deserialize, Default)]
struct Address {
  address: String,
  inner_path: String,
}

fn serve_site(req: HttpRequest) -> Result<NamedFile, ()> {
  info!("Serve site {}, {}",
    req.match_info().query("address"),
    req.match_info().query("inner_path"));
  let mut file_path = PathBuf::from("/home/crolsi/Programs/ZeroNet/data/")
    .join(PathBuf::from(req.match_info().query("address")))
    .join(PathBuf::from(req.match_info().query("inner_path")));
  info!("Path: {:?}", file_path);

  if file_path.is_dir() {
    file_path = file_path.join(PathBuf::from("index.html"));
  }

  let mut file = match NamedFile::open(file_path) {
    Ok(f) => f,
    Err(error) => {
      info!("Failed to get file: {:?}", error);
      return Result::Err(())
    }
  };

  Result::Ok(file)

  // let mut file = match File::open(file_path) {
  //   Ok(f) => f,
  //   Err(_) => return HttpResponse::Ok().body(format!("Could not load")),
  // };
  // let mut string = String::new();
  // file.read_to_string(&mut string);
  // HttpResponse::Ok().body(string)
  // HttpResponse::Ok().body(format!("Hello world! {}", args.address))
}

impl ZeroServer {
  pub fn new() -> ZeroServer {
    let http_server = HttpServer::new(|| {
      App::new()
        .route("/", web::get().to(index))
        .route("/{address}{inner_path:}", web::get().to(serve_site))
        .route("/{address}/{inner_path:.*}", web::get().to(serve_site))
      })
      .bind("127.0.0.1:42110")
      .unwrap()
      .run()
      .unwrap();

    ZeroServer {
      // http_server: http_server,
    }
  }
}
