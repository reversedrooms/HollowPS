use super::*;

pub async fn on_rpc_close_level_chg_tips(
    _session: &NetworkSession,
    _arg: &RpcCloseLevelChgTipsArg,
) -> Result<RpcCloseLevelChgTipsRet> {
    Ok(RpcCloseLevelChgTipsRet::new())
}

pub async fn on_rpc_del_new_map(
    _session: &NetworkSession,
    _arg: &RpcDelNewMapArg,
) -> Result<RpcDelNewMapRet> {
    Ok(RpcDelNewMapRet::new())
}
