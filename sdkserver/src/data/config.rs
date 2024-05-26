use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigInfo {
    pub min_version: String,
    pub latest_version: String,
    pub game_res_url: String,
    pub design_data_url: String,
    pub server_list_url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigInfoGroup {
    pub version_info_groups: HashMap<String, ConfigInfo>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AppConfig {
    pub info_groups: HashMap<String, ConfigInfoGroup>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerListInfo {
    pub sid: i32,
    pub server_name: String,
    pub ip: String,
    pub port: String,
    pub notice_region: String,
    pub protocol: String,
    #[serde(rename = "$Type")]
    ty: String,
}
