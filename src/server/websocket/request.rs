use crate::util::is_default;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum CommandType {
	// API
	AnnouncerInfo,
	CertAdd,
	CertSelect,
	ChannelJoin,
	DbQuery,
	DirList,
	FileDelete,
	FileGet,
	FileList,
	FileNeed,
	FileQuery,
	FileRules,
	FileWrite,
	Ping,
	ServerInfo,
	SiteInfo,
	SitePublish,
	SiteReload,
	SiteSign,
	SiteUpdate,
	UserGetGlobalSettings,
	UserSetGlobalSettings,
	// Bigfile
	BigFileUploadInit,
	// Cors
	CorsPermission,
	// Multiuser
	UserLoginForm,
	UserShowMasterSeed,
	// CryptMessage
	UserPublickey,
	EciesEncrypt,
	EciesDecrypt,
	AesEncrypt,
	AesDecrypt,
	// Newsfeed
	FeedFollow,
	FeedListFollow,
	FeedQuery,
	// MergerSite
	MergerSiteAdd,
	MergerSiteDelete,
	MergerSiteList,
	// Mute
	MuteAdd,
	MuteRemove,
	MuteList,
	// OptionalManager
	OptionalFileList,
	OptionalFileInfo,
	OptionalFilePin,
	OptionalFileUnpin,
	OptionalFileDelete,
	OptionalLimitStats,
	OptionalLimitSet,
	OptionalHelpList,
	OptionalHelp,
	OptionalHelpRemove,
	OptionalHelpAll,
	// Admin commands
	As,
	CertList,
	CertSet,
	ChannelJoinAllsite,
	ConfigSet,
	ServerPortcheck,
	ServerShutdown,
	ServerUpdate,
	SiteClone,
	SiteList,
	SitePause,
	SiteResume,

	ServerErrors,
	UserGetSettings,
	UserSetSettings,
	AnnouncerStats,
	SiteSetLimit,
	ChartDbQuery,
	FilterIncludeList,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
	pub cmd: CommandType,
	pub params: serde_json::Value,
	pub id: isize,
	#[serde(skip_serializing_if = "is_default", default)]
	pub wrapper_nonce: String,
}
