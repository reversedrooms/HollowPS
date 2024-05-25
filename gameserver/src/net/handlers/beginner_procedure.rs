use common::util;

use super::*;

const START_PROC_ID: i32 = 1;

pub async fn on_rpc_advance_beginner_procedure(
    session: &NetworkSession,
    arg: &RpcAdvanceBeginnerProcedureArg,
) -> Result<RpcAdvanceBeginnerProcedureRet> {
    let next_procedure_id = if arg.procedure_id == 0 {
        START_PROC_ID
    } else {
        arg.procedure_id + 1
    };

    tracing::info!("{arg:?}");
    if arg.procedure_id == 6 {
        Box::pin(world::enter_main_city(session)).await?;
    }

    Ok(RpcAdvanceBeginnerProcedureRet::new(next_procedure_id))
}

pub async fn on_rpc_beginnerbattle_begin(
    _session: &NetworkSession,
    arg: &RpcBeginnerbattleBeginArg,
) -> Result<RpcBeginnerbattleBeginRet> {
    Ok(RpcBeginnerbattleBeginRet::new(format!(
        "{}-{}",
        arg.battle_id,
        util::cur_timestamp_seconds()
    )))
}

pub async fn on_rpc_beginnerbattle_end(
    _session: &NetworkSession,
    arg: &RpcBeginnerbattleEndArg,
) -> Result<RpcBeginnerbattleEndRet> {
    tracing::info!("Battle statistics: {:?}", arg.battle_statistics);

    Ok(RpcBeginnerbattleEndRet::new())
}
