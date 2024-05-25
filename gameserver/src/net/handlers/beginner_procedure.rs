use common::util;

use super::*;

const START_PROC_ID: i32 = 1;

pub async fn on_rpc_advance_beginner_procedure_arg(
    session: &NetworkSession,
    arg: &RpcAdvanceBeginnerProcedureArg,
) -> Result<()> {
    let next_procedure_id = if arg.procedure_id == 0 {
        START_PROC_ID
    } else {
        arg.procedure_id + 1
    };

    tracing::info!("{arg:?}");
    if arg.procedure_id == 6 {
        Box::pin(world::enter_main_city(session)).await?;
    }

    session
        .send_rpc_ret(RpcAdvanceBeginnerProcedureRet::new(next_procedure_id))
        .await
}

pub async fn on_rpc_beginnerbattle_begin_arg(
    session: &NetworkSession,
    arg: &RpcBeginnerbattleBeginArg,
) -> Result<()> {
    session
        .send_rpc_ret(RpcBeginnerbattleBeginRet::new(format!(
            "{}-{}",
            arg.battle_id,
            util::cur_timestamp_seconds()
        )))
        .await
}

pub async fn on_rpc_beginnerbattle_end_arg(
    session: &NetworkSession,
    arg: &RpcBeginnerbattleEndArg,
) -> Result<()> {
    tracing::info!("Battle statistics: {:?}", arg.battle_statistics);

    session.send_rpc_ret(RpcBeginnerbattleEndRet::new()).await
}
