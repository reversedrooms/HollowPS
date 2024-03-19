use super::*;

pub async fn on_rpc_check_yorozuya_info_refresh_arg(
    session: &NetworkSession,
    _arg: &RpcCheckYorozuyaInfoRefreshArg,
) -> Result<()> {
    session
        .send_rpc_ret(RpcCheckYorozuyaInfoRefreshRet::new())
        .await
}
