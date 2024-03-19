use std::time::{SystemTime, UNIX_EPOCH};

use super::*;
use crate::game::util;

const DEFAULT_ACCOUNT_ID: u64 = 1337;

pub async fn on_rpc_login_arg(session: &NetworkSession, arg: &RpcLoginArg) -> Result<()> {
    tracing::info!("Received rpc login arg: {}", arg.account_name);
    *session.get_account_mut() = util::create_default_account(DEFAULT_ACCOUNT_ID);

    session
        .send_rpc_ret(RpcLoginRet::new(
            session.ns_prop_mgr.serialize_account_info(),
        ))
        .await
}

pub async fn on_ptc_get_server_timestamp_arg(
    session: &NetworkSession,
    _arg: &PtcGetServerTimestampArg,
) -> Result<()> {
    session
        .send_rpc_ret(PtcGetServerTimestampRet::new(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            0,
        ))
        .await
}

pub async fn on_rpc_keep_alive_arg(session: &NetworkSession, _arg: &RpcKeepAliveArg) -> Result<()> {
    session.send_rpc_ret(RpcKeepAliveRet::new()).await
}
