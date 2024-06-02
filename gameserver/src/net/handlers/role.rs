use super::*;

pub async fn on_rpc_mod_nick_name(
    session: &NetworkSession,
    arg: &RpcModNickNameArg,
) -> Result<RpcModNickNameRet> {
    tracing::info!("creating character");

    let player_info_changed = {
        let mut player = session.ns_prop_mgr.player_info.write();
        player.nick_name.replace(arg.nick_name.clone());
        player.avatar_id.replace(arg.avatar_id);

        PtcPlayerInfoChangedArg {
            player_uid: player.uid.unwrap(),
            player_info: PlayerInfo {
                nick_name: Some(arg.nick_name.clone()),
                avatar_id: Some(arg.avatar_id),
                ..Default::default()
            },
        }
    };

    session
        .send_rpc_arg(PTC_PLAYER_INFO_CHANGED_ID, &player_info_changed)
        .await?;
    Ok(RpcModNickNameRet::new())
}
