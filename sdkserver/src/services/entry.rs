use axum::Json;
use serde_json::json;

pub const ACCOUNT_TOKEN_ENDPOINT: &str = "/account/token";
pub const ACCOUNT_SERVER_ENDPOINT: &str = "/account/account_server";

#[tracing::instrument]
pub async fn account_token() -> Json<serde_json::Value> {
    tracing::info!("account_token");
    Json(json!({
    "ErrorCode": 0,
    "ErrorMsg": null,
    "Ext": {
        "Birthday": "01.01",
        "Country": "RU",
        "Token": "MostSecureTokenEver"
    }
    }))
}

#[tracing::instrument]
pub async fn account_server() -> Json<serde_json::Value> {
    tracing::info!("account_server");
    Json(json!({
    "ErrorCode": 0,
    "ErrorMsg": null,
    "Ext": {
        "Address": "127.0.0.1:10301/0"
    }
    }))
}
