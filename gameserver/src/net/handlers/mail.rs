use super::*;

pub async fn on_rpc_get_player_mails(
    _session: &NetworkSession,
    _arg: &RpcGetPlayerMailsArg,
) -> Result<RpcGetPlayerMailsRet> {
    Ok(RpcGetPlayerMailsRet::new(0))
}
