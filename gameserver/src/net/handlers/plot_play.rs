use super::*;

pub async fn on_rpc_perform_trigger_arg(
    session: &NetworkSession,
    arg: &RpcPerformTriggerArg,
) -> Result<()> {
    session
        .send_rpc_ret(RpcPerformTriggerRet::new(format!(
            "{}-{}",
            arg.perform_id, arg.perform_type
        )))
        .await
}

pub async fn on_rpc_perform_end_arg(
    session: &NetworkSession,
    _arg: &RpcPerformEndArg,
) -> Result<()> {
    session.send_rpc_ret(RpcPerformEndRet::new()).await
}

pub async fn on_rpc_finish_a_c_t_perform_show_arg(
    session: &NetworkSession,
    _arg: &RpcFinishACTPerformShowArg,
) -> Result<()> {
    session
        .send_rpc_ret(RpcFinishACTPerformShowRet::new())
        .await
}

pub async fn on_rpc_perform_jump_arg(
    session: &NetworkSession,
    _arg: &RpcPerformJumpArg,
) -> Result<()> {
    session.send_rpc_ret(RpcPerformJumpRet::new()).await
}
