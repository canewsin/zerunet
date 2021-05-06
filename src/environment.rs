use clap::{App, Arg, SubCommand};
use std::path::PathBuf;
use std::str::FromStr;

const TRACKERS: &[&str] = &[
	"zero://127.0.0.1:15442",
	"zero://38.39.233.117:15441",
	"zero://125.123.63.229:26552",
	"zero://149.248.37.249:13601",
	"zero://211.125.90.79:11110",
	"zero:/45.77.23.92:15555",
	"zero://45.76.157.51:19355",
	"zero://96.21.169.17:28969",
	"zero://51.38.34.170:15656",
	"zero://boot3rdez4rzn36x.onion:15441",
	// "zero://zero.booth.moe#f36ca555bee6ba216b14d10f38c16f7769ff064e0e37d887603548cc2e64191d:443",
	// "udp://tracker.coppersurfer.tk:6969",
	// "udp://amigacity.xyz:6969",
	// "udp://104.238.198.186:8000",
	// "http://tracker01.loveapp.com:6789/announce",
	// "http://open.acgnxtracker.com:80/announce",
	// "http://open.trackerlist.xyz:80/announce",
	// "zero://2602:ffc5::c5b2:5360:26312",
];

#[derive(Debug)]
pub struct Environment {
	pub data_path: PathBuf,
	pub broadcast_port: usize,
	pub ui_ip: String,
	pub ui_port: usize,
	pub trackers: Vec<String>,
}

#[derive(Debug)]
pub struct Error {
	string: String,
}

impl Error {
	fn from_str(string: &str) -> Error {
		Error {
			string: String::from(string),
		}
	}
}

impl From<std::num::ParseIntError> for Error {
	fn from(err: std::num::ParseIntError) -> Error {
		Error::from_str(&format!("{:?}", err))
	}
}

pub fn get_env() -> Result<Environment, Error> {
	let matches = App::new("zerunet")
		.version("0.1.0")
		.author("Ansho Rei <anshorei@zeroid.bit>")
		.about("A ZeroNet implementation written in Rust.")
		.args(&[
			// Should probably be removed in favor of environment flags
			Arg::with_name("VERBOSE")
				.short("v")
				.long("verbose")
				.help("More detailed logging"),
			// Should probably be replaced with arguments dealing particularly with coffeescript compilation and other debug features
			Arg::with_name("DEBUG").long("debug").help("Debug mode"),
			// Should probably be removed in favor of environment flags
			Arg::with_name("SILENT")
				.long("silent")
				.help("Only log errors to terminal"),
			// Look up what this does:
			Arg::with_name("DEBUG_SOCKET")
				.long("debug_socket")
				.help("Debug socket connections"),
			Arg::with_name("MERGE_MEDIA")
				.long("merge_media")
				.help("Merge all.js and all.css"),
			Arg::with_name("BATCH")
				.long("batch")
				.help("Batch mode (No interactive input for commands)"),
			Arg::with_name("CONFIG_FILE")
				.long("config_file")
				.default_value("./zeronet.conf")
				.help("Path of config file"),
			Arg::with_name("DATA_DIR")
				.long("data_dir")
				.default_value("./data")
				.help("Path of data directory"),
			// Should be removed
			Arg::with_name("CONSOLE_LOG_LEVEL")
				.long("console_log_level")
				.help("Level of logging to file"),
			Arg::with_name("LOG_DIR")
				.long("log_dir")
				.default_value("./log")
				.help("Path of logging directory"),
			Arg::with_name("LOG_LEVEL")
				.long("log_level")
				.help("Level of loggin to file"),
			Arg::with_name("LOG_ROTATE")
				.long("log_rotate")
				.default_value("daily")
				.possible_values(&["hourly", "daily", "weekly", "off"])
				.help("Log rotate interval"),
			Arg::with_name("LOG_ROTATE_BACKUP_COUNT")
				.long("log_rotate_backup_count")
				.default_value("5")
				.help("Log rotate backup count"),
			Arg::with_name("LANGUAGE")
				.short("l")
				.long("language")
				.default_value("en")
				.help("Web interface language"),
			Arg::with_name("UI_IP")
				.long("ui_ip")
				.default_value("127.0.0.1")
				.help("Web interface bind address"),
			Arg::with_name("UI_PORT")
				.long("ui_port")
				.default_value("43110")
				.help("Web interface bind port"),
			Arg::with_name("UI_RESTRICT")
				.long("ui_restrict")
				.help("Restrict web access"),
			Arg::with_name("UI_HOST")
				.long("ui_host")
				.help("Allow access using this hosts"),
			Arg::with_name("UI_TRANS_PROXY")
				.long("ui_trans_proxy")
				.help("Allow access using a transparent proxy"),
			Arg::with_name("OPEN_BROWSER")
				.long("open_browser")
				.help("Open homepage in web browser automatically"),
			Arg::with_name("HOMEPAGE")
				.long("homepage")
				.default_value("1HeLLo4uzjaLetFx6NH3PMwFP3qbRbTf3D")
				.help("Web interface Homepage"),
			// UPDATE SITE?
			Arg::with_name("DIST_TYPE")
				.long("dist_type")
				.default_value("source")
				.help("Type of installed distribution"),
			Arg::with_name("SIZE_LIMIT")
				.long("size_limit")
				.default_value("10")
				.help("Default site size limit in MB"),
			Arg::with_name("FILE_SIZE_LIMIT")
				.long("file_size_limit")
				.default_value("10")
				.help("Maximum per file size limit"),
			Arg::with_name("CONNECTED_LIMIT")
				.long("connected_limit")
				.default_value("8")
				.help("Max connected peer per site"),
			Arg::with_name("GLOBAL_CONNECTED_LIMIT")
				.long("global_connected_limit")
				.default_value("512")
				.help("Max connections"),
			Arg::with_name("FILESERVER_IP")
				.long("fileserver_ip")
				.default_value("*")
				.help("Fileserver bind address"),
			Arg::with_name("FILESERVER_PORT_RANGE")
				.long("fileserver_port_range")
				.default_value("10000-40000")
				.help("Fileserver randomization range"),
			Arg::with_name("FILESERVER_IP_TYPE")
				.long("fileserver_ip_type")
				.default_value("dual")
				.possible_values(&["ipv4", "ipv6", "dual"])
				.help("Fileserver ip type"),
			Arg::with_name("IP_LOCAL")
				.long("ip_local")
				.default_value("['127.0.0.1', '::1']")
				.help("My local ips"),
			Arg::with_name("IP_EXTERNAL")
				.long("ip_external")
				.default_value("[]")
				.help("Set reported external ip"),
			Arg::with_name("TOR_HS_PORT")
				.long("tor_hs_port")
				.default_value("15441")
				.help("Hidden service port in Tor always mode"),
			Arg::with_name("BROADCAST_PORT")
				.long("broadcast_port")
				.default_value("1544")
				.help("Port to broadcast local discovery messages"),
		])
		.subcommands(vec![
			SubCommand::with_name("siteCreate"),
			SubCommand::with_name("siteNeedFile"),
			SubCommand::with_name("siteDownload"),
			SubCommand::with_name("siteSign"),
			SubCommand::with_name("sitePublish"),
			SubCommand::with_name("siteVerify"),
			SubCommand::with_name("siteCmd"),
			SubCommand::with_name("dbRebuild"),
			SubCommand::with_name("dbQuery"),
		])
		.get_matches();

	let data_path = PathBuf::from_str(matches.value_of("DATA_DIR").unwrap()).unwrap();
	if !data_path.is_dir() {
		return Err(Error::from_str("DATA_DIR does not exist"));
	}
	let ui_ip = matches.value_of("UI_IP").unwrap();
	let ui_port: usize = matches.value_of("UI_PORT").unwrap().parse()?;
	let broadcast_port: usize = matches.value_of("BROADCAST_PORT").unwrap().parse()?;
	let env = Environment {
		data_path,
		broadcast_port,
		ui_ip: String::from(ui_ip),
		ui_port,
		trackers: TRACKERS.iter().map(|s| String::from(*s)).collect(),
	};
	Ok(env)
}
