use actix_web::{
  HttpRequest, Result, HttpResponse,
  web::{Data, Query}
};
use actix_files::NamedFile;
use log::*;
use std::path::{PathBuf};
use crate::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Info {
  nonce: String,
}

pub fn serve_file(req: &HttpRequest, _data: Data<crate::server::ZeroServer>) -> Result<NamedFile, Error> {
  
  let mut file_path = PathBuf::from("/home/crolsi/Programs/ZeroNet/data/")
    .join(PathBuf::from(req.match_info().query("address")))
    .join(PathBuf::from(req.match_info().query("inner_path")));

  if file_path.is_dir() {
    file_path = file_path.join(PathBuf::from("index.html"));
  }

  trace!("Serve file zero://{}/{}",
    req.match_info().query("address"),
    req.match_info().query("inner_path"));

  let file = NamedFile::open(file_path)?;
  Result::Ok(file)

  // let mut file = match File::open(&file_path) {
  //   Ok(f) => f,
  //   Err(error) => {
  //     error!("Error handling request: failed to get {:?}: {:?}", file_path, error);
  //     return HttpResponse::from("not found");
  //   }
  // };
  // let mut string = Vec::new();
  // match file.read_to_end(&mut string) {
  //   Ok(_) => {},
  //   Err(err) => error!("Failed to read to string {:?}", err),
  // };

  // HttpResponse::Ok()
  //   .header("X-Hdr", "sample")
  //   .content_type("")
  //   .body(string)



  // let mut file = match File::open(file_path) {
  //   Ok(f) => f,
  //   Err(_) => return HttpResponse::Ok().body(format!("Could not load")),
  // };
  // let mut string = String::new();
  // file.read_to_string(&mut string);
  // HttpResponse::Ok().body(string)
  // HttpResponse::Ok().body(format!("Hello world! {}", args.address))
}
