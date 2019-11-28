use super::{SERVER_URL, SERVER_PORT};
use actix_web::{Result, HttpRequest, HttpResponse, Responder};
use actix_files::NamedFile;
use log::*;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::Read;
use uuid::Uuid;
use crate::error::Error;

struct WrapperData {
  inner_path: String,
  file_url: String, //escape
  file_inner_path: String, //Escape
  address: String,
  title: String, //Escape
  body_style: String,
  meta_tags: String,
  query_string: String, //Escape
  wrapper_key: String,
  ajax_key: String,
  wrapper_nonce: String,
  postmessage_nonce_security: String,
  permissions: String,
  show_loadingscreen: String,
  sandbox_permissions: String,
  rev: String,
  lang: String,
  homepage: String,
  themeclass: String,
  script_nonce: String,
}

pub fn serve_wrapper(req: HttpRequest, data: actix_web::web::Data<crate::server::ZeroServer>) -> HttpResponse {
  let nonce = Uuid::new_v4().to_simple().to_string();

  {
    let mut nonces = data.wrapper_nonces.lock().unwrap();
    nonces.insert(nonce.clone());
    trace!("Valid nonces ({}): {:?}", nonces.len(), nonces);
  }

  let address = req.match_info().query("address");
  let inner_path = req.match_info().query("inner_path");
  info!("Serving wrapper for zero://{}/{}", address, inner_path);

  let path = PathBuf::from("./ui/wrapper.html");
  let string = match render(&path, WrapperData {
    inner_path: String::from(inner_path),
    file_url: format!("\\/{}\\/", address),
    file_inner_path: String::from(inner_path),
    address: String::from(address),
    title: String::from("zerunet test site"),
    body_style: String::from("body_style"),
    meta_tags: String::from("<test>"),
    query_string: format!("\\?wrapper_nonce\\={}", nonce.clone()),
    wrapper_key: String::from("wrapper_key"),
    ajax_key: String::from("ajax_key"),
    wrapper_nonce: nonce.clone(),
    postmessage_nonce_security: String::from("true"),
    permissions: String::from("[]"),
    show_loadingscreen: String::from("false"),
    sandbox_permissions: String::from("allow-popups-to-escape-sandbox"),
    rev: String::from("rev"),
    lang: String::from("lang"),
    homepage: String::from("/1HeLLo4uzjaLetFx6NH3PMwFP3qbRbTf3D"),
    themeclass: String::from("themeclass"),
    script_nonce: String::from("script_nonce"),
  }) {
    Ok(s) => s,
    Err(_) => String::new(),
  };
  HttpResponse::Ok()
    .content_type("html")
    .header("X-Hdr", "sample")
    .body(string)
}

// render loads the wrapper and applies the wrapperdata to it
fn render(file_path: &Path, data: WrapperData) -> Result<String,()> {
  let mut file = match File::open(file_path) {
    Ok(f) => f,
    Err(error) => {
      error!("Failed to get file: {:?}", error);
      return Result::Err(())
    }
  };
  let mut string = String::new();
  match file.read_to_string(&mut string) {
    Ok(_) => {},
    Err(_) => return Err(()),
  };
  let server_url = format!("{}:{}", SERVER_URL, SERVER_PORT);
  string = string.replace("{server_url}", &server_url.as_str());
  string = string.replace("{inner_path}", &data.inner_path);
  string = string.replace("{file_url}", &data.file_url);
  string = string.replace("{file_inner_path}", &data.file_inner_path);
  string = string.replace("{address}", &data.address);
  string = string.replace("{title}", &data.title);
  string = string.replace("{body_style}", &data.body_style);
  string = string.replace("{meta_tags}", &data.meta_tags);
  string = string.replace("{query_string}", &data.query_string);
  string = string.replace("{wrapper_key}", &data.wrapper_key);
  string = string.replace("{ajax_key}", &data.ajax_key);
  string = string.replace("{wrapper_nonce}", &data.wrapper_nonce);
  string = string.replace("{postmessage_nonce_security}", &data.postmessage_nonce_security);
  string = string.replace("{permissions}", &data.permissions);
  string = string.replace("{show_loadingscreen}", &data.show_loadingscreen);
  string = string.replace("{sandbox_permissions}", &data.sandbox_permissions);
  string = string.replace("{rev}", &data.rev);
  string = string.replace("{lang}", &data.lang);
  string = string.replace("{homepage}", &data.homepage);
  string = string.replace("{themeclass}", &data.themeclass);
  string = string.replace("{script_nonce}", &data.script_nonce);
  return Ok(string)
}

pub fn serve_uimedia(req: HttpRequest) -> HttpResponse {
  let inner_path = req.match_info().query("inner_path");
  match serve_uimedia_file(inner_path) {
    Ok(f) => match f.respond_to(&req) {
      Ok(r) => r,
      Err(_) => HttpResponse::BadRequest().finish(),
    },
    Err(_) => HttpResponse::BadRequest().finish(),
  }
}

pub fn serve_uimedia_file(inner_path: &str) -> Result<NamedFile, Error> {
  // trace!("Serving uimedia file: {:?}", inner_path);
  let mut file_path = PathBuf::from("./ui/media");
  
  match inner_path {
    "favicon.ico" | "apple-touch-icon.png" => file_path.push(&Path::new("img")),
    _ => {},
  }
  file_path.push(&Path::new(inner_path));
  
  if !file_path.is_file() {
    return Err(Error::FileNotFound);
  }
  let f = NamedFile::open(file_path)?;

  Ok(f)
}
