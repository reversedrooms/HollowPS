use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct HttpRet<Ext> {
    pub error_code: i32,
    pub error_msg: Option<String>,
    pub ext: Ext,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntryTokenRet {
    pub birthday: String,
    pub country: String,
    pub token: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntryAccountServerRet {
    pub address: String,
}
