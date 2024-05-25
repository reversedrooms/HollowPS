use super::*;

pub async fn on_rpc_check_yorozuya_info_refresh(
    _session: &NetworkSession,
    _arg: &RpcCheckYorozuyaInfoRefreshArg,
) -> Result<RpcCheckYorozuyaInfoRefreshRet> {
    Ok(RpcCheckYorozuyaInfoRefreshRet::new())
}
