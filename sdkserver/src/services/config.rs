use std::sync::LazyLock;

use axum::{body::Body, response::IntoResponse};
use rand::Rng;
use serde::de::DeserializeOwned;

use crate::crypto;
use crate::data::config::*;

pub const APP_CONFIG_ENDPOINT: &str = "/design_data/NAP_Publish_AppStore_0.1.0/oversea/config.bin";
pub const SERVER_LIST_ENDPOINT: &str =
    "/design_data/NAP_Publish_AppStore_0.1.0/oversea/serverlist.bin";
pub const VERSIONS_BUNDLE_ENDPOINT: &str = "/game_res/NAP_Publish/output_147608_1361f678bc/client/StandaloneWindows64/oversea/versions.bundle";

static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(|| read_config("config.json"));
static SERVER_LIST: LazyLock<Vec<ServerListInfo>> =
    LazyLock::new(|| read_config("serverlist.json"));

static VERSIONS_BUNDLE: LazyLock<Box<[u8]>> = LazyLock::new(|| read_binary_data("versions.bundle"));

pub async fn application() -> Vec<u8> {
    crypto::encrypt_config(
        serde_json::to_string(&*APP_CONFIG).unwrap().as_str(),
        &random_xorpad(16),
    )
}

pub async fn server_list() -> Vec<u8> {
    crypto::encrypt_config(
        serde_json::to_string(&*SERVER_LIST).unwrap().as_str(),
        &random_xorpad(16),
    )
}

pub async fn versions_bundle() -> impl IntoResponse {
    Body::from(&**VERSIONS_BUNDLE)
}

fn read_config<T>(file: &str) -> T
where
    T: DeserializeOwned,
{
    let data = std::fs::read_to_string(format!("assets/Application/{file}")).unwrap();
    serde_json::from_str::<T>(&data).unwrap()
}

fn read_binary_data(file: &str) -> Box<[u8]> {
    std::fs::read(format!("assets/Application/{file}"))
        .unwrap()
        .into()
}

fn random_xorpad(len: usize) -> String {
    const CHARSET: &[u8] = b"0123456789";

    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
