use super::*;

pub async fn on_rpc_battle_report_arg(
    session: &NetworkSession,
    arg: &RpcBattleReportArg,
) -> Result<()> {
    let need_index = arg
        .battle_reports
        .last()
        .map_or(0, |report| report.index + 1);

    session
        .send_rpc_ret(RpcBattleReportRet::new(need_index))
        .await
}
