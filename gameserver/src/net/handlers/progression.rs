use super::*;

pub async fn on_rpc_close_level_chg_tips_arg(
    session: &NetworkSession,
    _arg: &RpcCloseLevelChgTipsArg,
) -> Result<()> {
    session.send_rpc_ret(RpcCloseLevelChgTipsRet::new()).await
}
