use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
struct Request {
  cmd: String,
  params: Vec<String>,
}

fn handle_request(req: Request) {

}

// fn action_as()
// fn action_response()
// fn action_ping()
// fn action_site_info()
// fn action_site_bad_files()
// fn action_channel_join()
// fn actionServerInfo()
// fn actionServerGetWrapper()
// fn actionAnnounceInfo()
// fn actionAnnouncerStatus()
// fn actionSiteSign()
// fn actionSitePublish()
// fn actionSiteReload()
// fn actionFileDelete()
// fn actionFileQuery()
// fn actionFileList()
// fn actionDirList()
// fn actionDbQuery()
// fn actionFileGet()
// fn actionFileNeed()
// fn actionFileRules()
// fn actionCertAdd()
// fn actionCertSelect()
// fn actionPermissionAdd()
// fn actionPermissionRemove()
// fn actionPermissionDetails()
// fn actionCertSet()
// fn actionCertList()
// fn actionSiteList()
// fn actionChannelJoinAllsite()
// fn actionSiteUpdate()
// fn actionSitePause()
// fn actionSiteResume()
// fn actionSiteDelete()
// fn actionSiteClone()
// fn actionSiteSetLimit()
// fn actionSiteAdd()
// fn actionSiteListModifiedFiles()
// fn actionSiteSetSettingsValue()
// fn actionUserGetSettings()
// fn actionUserSetSettings()
// fn actionUserGetGlobalSettings()
// fn actionUserSetGlobalSettings()
// fn actionServerUpdate()
// fn actionServerPortcheck()
// fn actionServerShutdown()
// fn actionServerShowdirectory()
// fn actionConfigSet()
