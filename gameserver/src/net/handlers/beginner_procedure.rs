use std::time::{SystemTime, UNIX_EPOCH};

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
    let cur_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    session
        .send_rpc_ret(RpcBeginnerbattleBeginRet::new(format!(
            "{cur_timestamp}-{}",
            arg.battle_id
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
