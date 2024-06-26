use std::time::{SystemTime, UNIX_EPOCH};

use super::*;
use crate::{
    game::util,
    net::session::{AccountUID, PlayerUID},
};

const DEFAULT_ACCOUNT_ID: u64 = 1;

pub async fn on_rpc_login(session: &NetworkSession, arg: &RpcLoginArg) -> Result<RpcLoginRet> {
    tracing::info!("Received rpc login arg: {}", arg.account_name);

    match session.logged_in(
        AccountUID(DEFAULT_ACCOUNT_ID),
        util::create_default_account(DEFAULT_ACCOUNT_ID),
    ) {
        Ok(()) => Ok(RpcLoginRet::new(
            session.ns_prop_mgr.serialize_account_info(),
        )),
        Err(_) => Ok(RpcLoginRet::error(ErrorCode::RepeatedLogin, Vec::new())),
    }
}

pub async fn on_rpc_create_player(
    session: &NetworkSession,
    _arg: &RpcCreatePlayerArg,
) -> Result<RpcCreatePlayerRet> {
    let account_uid = session.account_uid();
    let player_count = session
        .ns_prop_mgr
        .account_info
        .read()
        .players
        .as_ref()
        .unwrap()
        .len();

    let player_uid = PlayerUID::new(account_uid, player_count + 1);
    session
        .ns_prop_mgr
        .account_info
        .write()
        .players
        .as_mut()
        .unwrap()
        .push(player_uid.raw());

    Ok(RpcCreatePlayerRet::new(player_uid.raw()))
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
