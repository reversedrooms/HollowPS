use axum::{body::Body, response::IntoResponse};
use serde_json::json;

use crate::crypto;

pub const APP_CONFIG_ENDPOINT: &str = "/design_data/NAP_Publish_AppStore_0.1.0/oversea/config.bin";
pub const SERVER_LIST_ENDPOINT: &str =
    "/design_data/NAP_Publish_AppStore_0.1.0/oversea/serverlist.bin";
pub const VERSIONS_BUNDLE_ENDPOINT: &str = "/game_res/NAP_Publish/output_147608_1361f678bc/client/StandaloneWindows64/oversea/versions.bundle";

pub async fn application() -> Vec<u8> {
    crypto::encrypt_config(
        json!({
            "InfoGroups": {
                "StandaloneWindows64": {
                    "VersionInfoGroups": {
                        "0.1.0": {
                            "MinVersion": "0.1.0",
                            "LatestVersion": "0.1.0",
                            "GameResUrl": "http://127.0.0.1:21000/game_res/NAP_Publish/output_147608_1361f678bc/client/",
                            "DesignDataUrl": "http://127.0.0.1:21000/",
                            "ServerListUrl": "http://127.0.0.1:21000/design_data/NAP_Publish_AppStore_0.1.0/oversea/serverlist.bin",
                            "$Type": "Foundation.ConfigurationInfo"
                        },
                    },
                    "$Type": "Foundation.ConfigurationInfoGroup"
                },
            },
        })
        .to_string()
        .as_str(),
        "MostSecureKey",
    )
}

pub async fn server_list() -> Vec<u8> {
    crypto::encrypt_config(
        json!([{
            "sid": 142,
            "serverName": "HollowPS",
            "ip": "127.0.0.1",
            "port": "21000",
            "noticeRegion": "nap_glb_cbus01",
            "protocol": "http",
            "$Type": "MoleMole.ServerListInfo"
        }])
        .to_string()
        .as_str(),
        "MostSecureKey",
    )
}

pub const VERSION_BUNDLE: &[u8] = include_bytes!("../../versions.bundle");

pub async fn versions_bundle() -> impl IntoResponse {
    Body::from(VERSION_BUNDLE)
}
