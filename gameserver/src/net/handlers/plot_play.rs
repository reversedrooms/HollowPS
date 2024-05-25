use super::*;

pub async fn on_rpc_perform_trigger(
    _session: &NetworkSession,
    arg: &RpcPerformTriggerArg,
) -> Result<RpcPerformTriggerRet> {
    Ok(RpcPerformTriggerRet::new(format!(
        "{}-{}",
        arg.perform_id, arg.perform_type
    )))
}

pub async fn on_rpc_perform_end(
    _session: &NetworkSession,
    _arg: &RpcPerformEndArg,
) -> Result<RpcPerformEndRet> {
    Ok(RpcPerformEndRet::new())
}

pub async fn on_rpc_finish_act_perform_show(
    _session: &NetworkSession,
    _arg: &RpcFinishActPerformShowArg,
) -> Result<RpcFinishActPerformShowRet> {
    Ok(RpcFinishActPerformShowRet::new())
}

pub async fn on_rpc_perform_jump(
    _session: &NetworkSession,
    _arg: &RpcPerformJumpArg,
) -> Result<RpcPerformJumpRet> {
    Ok(RpcPerformJumpRet::new())
}
