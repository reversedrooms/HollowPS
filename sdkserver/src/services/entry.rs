use crate::data::entry::*;
use axum::Json;

pub const ACCOUNT_TOKEN_ENDPOINT: &str = "/account/token";
pub const ACCOUNT_SERVER_ENDPOINT: &str = "/account/account_server";

#[tracing::instrument]
pub async fn account_token() -> Json<HttpRet<EntryTokenRet>> {
    tracing::info!("account_token");

    Json(HttpRet {
        error_code: 0,
        error_msg: None,
        ext: EntryTokenRet {
            birthday: String::from("01.01"),
            country: String::from("RU"),
            token: String::from("MostSecureTokenEver"),
        },
    })
}

#[tracing::instrument]
pub async fn account_server() -> Json<HttpRet<EntryAccountServerRet>> {
    tracing::info!("account_server");

    Json(HttpRet {
        error_code: 0,
        error_msg: None,
        ext: EntryAccountServerRet {
            address: String::from("127.0.0.1:10301/0"),
        },
    })
}
