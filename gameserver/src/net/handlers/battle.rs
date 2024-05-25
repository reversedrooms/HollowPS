use super::*;

pub async fn on_rpc_battle_report(
    _session: &NetworkSession,
    arg: &RpcBattleReportArg,
) -> Result<RpcBattleReportRet> {
    let need_index = arg
        .battle_reports
        .last()
        .map_or(0, |report| report.index + 1);

    Ok(RpcBattleReportRet::new(need_index))
}
