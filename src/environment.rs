use clap::{App, Arg, SubCommand};

pub struct Environment {}

pub fn get_env() -> Environment {
	let matches = App::new("gcrunner")
		.version("0.1")
		.author("Ansho Rei <anshorei@zeroid.bit>")
		.about("Does awesome things")
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
		])
		.get_matches();
	Environment {}
}