use env_logger::Builder;
use futures::executor::block_on;
use log::Level;
use reqwest;

#[inline]
pub fn init() {
	try_init().unwrap();
}

pub fn try_init() -> Result<(), log::SetLoggerError> {
	try_init_custom_env("RUST_LOG")
}

pub fn try_init_custom_env(environment_variable_name: &str) -> Result<(), log::SetLoggerError> {
	let mut builder = formatted_builder();

	if let Ok(s) = ::std::env::var(environment_variable_name) {
		builder.parse_filters(&s);
	}

	builder.try_init()
}

pub fn formatted_builder() -> Builder {
	let mut builder = Builder::new();

	builder.format(|f, record| {
    use std::io::Write;
    let target = record.target();
    let level = record.level();
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:9999/api/v2/write?org=zerunet&bucket=zeronet&precision=s")
      .header(reqwest::header::AUTHORIZATION, "Token hgt8JHm1c6c9_rD_lumpXNEf1qCjVqyT13AOSzrlbZfhlKEIc5MaMfKgZq8H4w1wHDCsFICF-UGEI3Zok5OiMg==")
      .body(format!("log,appname=zerunet,facility=home,host=dell,level={} trace=\"{}\",message=\"{}\"", level, target, record.args()))
      .send();

    match block_on(res) {
      Ok(_) => Ok(()),
      Err(error) => {
        println!("Error: {:?}", error);
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound))
      },
    }
  });

	builder
}
