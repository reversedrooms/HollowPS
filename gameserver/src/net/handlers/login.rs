use std::time::{SystemTime, UNIX_EPOCH};

use super::*;
use crate::game::util;

const DEFAULT_ACCOUNT_ID: u64 = 1337;

pub async fn on_rpc_login(session: &NetworkSession, arg: &RpcLoginArg) -> Result<RpcLoginRet> {
    tracing::info!("Received rpc login arg: {}", arg.account_name);
    *session.get_account_mut() = util::create_default_account(DEFAULT_ACCOUNT_ID);

    Ok(RpcLoginRet::new(
        session.ns_prop_mgr.serialize_account_info(),
    ))
}

pub async fn on_ptc_get_server_timestamp(
    _session: &NetworkSession,
    _arg: &PtcGetServerTimestampArg,
) -> Result<PtcGetServerTimestampRet> {
    Ok(PtcGetServerTimestampRet::new(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
        0,
    ))
}

pub async fn on_rpc_keep_alive(
    _session: &NetworkSession,
    _arg: &RpcKeepAliveArg,
) -> Result<RpcKeepAliveRet> {
    Ok(RpcKeepAliveRet::new())
}
