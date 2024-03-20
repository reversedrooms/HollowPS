use super::*;

pub async fn on_rpc_mod_nick_name_arg(
    session: &NetworkSession,
    arg: &RpcModNickNameArg,
) -> Result<()> {
    tracing::info!("creating character");

    let mut player = session.get_player_mut();
    player.nick_name.replace(arg.nick_name.clone());
    player.avatar_id.replace(arg.avatar_id);

    let player_info_changed = PtcPlayerInfoChangedArg {
        player_uid: player.uid.unwrap(),
        player_info: PlayerInfo {
            nick_name: Some(arg.nick_name.clone()),
            avatar_id: Some(arg.avatar_id),
            ..Default::default()
        },
    };

    session.send_rpc_arg(101, &player_info_changed).await?;
    session.send_rpc_ret(RpcModNickNameRet::new()).await
}
