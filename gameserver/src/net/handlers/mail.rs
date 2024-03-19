use super::*;

pub async fn on_rpc_get_player_mails_arg(
    session: &NetworkSession,
    _arg: &RpcGetPlayerMailsArg,
) -> Result<()> {
    session.send_rpc_ret(RpcGetPlayerMailsRet::new(0)).await
}
