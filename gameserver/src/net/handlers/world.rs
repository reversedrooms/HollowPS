use qwer::{
    pdkhashmap, phashmap, phashset, PropertyDoubleKeyHashMap, PropertyHashMap, PropertyHashSet,
};

use crate::config::CONFIGURATION;
use crate::data;
use crate::game::util;
use crate::net::session::PlayerUID;

use super::*;

pub async fn on_rpc_run_event_graph(
    session: &NetworkSession,
    arg: &RpcRunEventGraphArg,
) -> Result<RpcRunEventGraphRet> {
    let unit = session.context.scene_unit_manager.get(arg.owner_uid);

    let interact_id = match unit {
        Some(SceneUnitProtocolInfo::NpcProtocolInfo { interacts_info, .. })
            if interacts_info.len() != 0 =>
        {
            *interacts_info.iter().next().unwrap().0
        }
        Some(SceneUnitProtocolInfo::NpcProtocolInfo { tag, id, .. }) => {
            let main_city_object = data::get_main_city_object(tag, id).unwrap();
            *main_city_object.default_interact_ids.first().unwrap()
        }
        None => {
            return Ok(RpcRunEventGraphRet::error(
                ErrorCode::EntityNotExist,
                Vec::new(),
            ))
        }
    };

    let mut ptc_sync_event_info = PtcSyncEventInfoArg {
        owner_type: arg.owner_type,
        owner_uid: arg.owner_uid,
        updated_events: pdkhashmap![],
    };

    ptc_sync_event_info.updated_events.insert(
        interact_id,
        100,
        EventInfo {
            id: 100,
            cur_action_id: 101,
            action_move_path: Vec::from([101, 102, 101]),
            state: EventState::Initing,
            prev_state: EventState::WaitingClient,
            cur_action_info: ActionInfo::None {},
            cur_action_state: ActionState::Init,
            predicated_failed_actions: phashset![],
            stack_frames: Vec::new(),
        },
    );

    session
        .push_rpc_arg(PTC_SYNC_EVENT_INFO_ID, ptc_sync_event_info)
        .await?;

    Ok(RpcRunEventGraphRet::new())
}

pub async fn on_rpc_finish_event_graph_perform_show(
    session: &NetworkSession,
    arg: &RpcFinishEventGraphPerformShowArg,
) -> Result<RpcFinishEventGraphPerformShowRet> {
    let mut ptc_sync_event_info = PtcSyncEventInfoArg {
        owner_type: EventGraphOwnerType::SceneUnit,
        owner_uid: arg.owner_uid,
        updated_events: pdkhashmap![],
    };

    ptc_sync_event_info.updated_events.insert(
        arg.event_graph_id,
        arg.event_id,
        EventInfo {
            id: arg.event_id,
            cur_action_id: -1,
            action_move_path: Vec::from([101, 102, 101, -1]),
            state: EventState::Finished,
            prev_state: EventState::Initing,
            cur_action_info: ActionInfo::None {},
            cur_action_state: ActionState::Init,
            predicated_failed_actions: phashset![],
            stack_frames: Vec::new(),
        },
    );

    session
        .push_rpc_arg(PTC_SYNC_EVENT_INFO_ID, ptc_sync_event_info)
        .await?;

    Ok(RpcFinishEventGraphPerformShowRet::new())
}

pub async fn on_rpc_interact_with_unit(
    session: &NetworkSession,
    arg: &RpcInteractWithUnitArg,
) -> Result<RpcInteractWithUnitRet> {
    tracing::info!(
        "InteractWithUnit: unit_uid: {}, interaction: {}",
        arg.unit_uid,
        arg.interaction
    );

    let unit = session.context.scene_unit_manager.get(arg.unit_uid);

    let interact_id = match unit {
        Some(SceneUnitProtocolInfo::NpcProtocolInfo { interacts_info, .. })
            if interacts_info.len() != 0 =>
        {
            *interacts_info.iter().next().unwrap().0
        }
        Some(SceneUnitProtocolInfo::NpcProtocolInfo { tag, id, .. }) => {
            let main_city_object = data::get_main_city_object(tag, id).unwrap();
            *main_city_object.default_interact_ids.first().unwrap()
        }
        None => {
            return Ok(RpcInteractWithUnitRet::error(
                ErrorCode::EntityNotExist,
                Vec::new(),
            ))
        }
    };

    // 0 - ServerInteractiveSystem::TriggerEnterEvent
    // 1 - ServerInteractiveSystem::TriggerExitEvent
    // 2 - ServerInteractiveSystem::TriggerInteractDirectly
    if arg.interaction == 2 {
        let mut ptc_sync_event_info = PtcSyncEventInfoArg {
            owner_type: EventGraphOwnerType::SceneUnit,
            owner_uid: arg.unit_uid,
            updated_events: pdkhashmap![],
        };

        ptc_sync_event_info.updated_events.insert(
            interact_id,
            100,
            EventInfo {
                id: 100,
                cur_action_id: 101,
                action_move_path: Vec::from([101]),
                state: EventState::WaitingClient,
                prev_state: EventState::Running,
                cur_action_info: ActionInfo::None {},
                cur_action_state: ActionState::Init,
                predicated_failed_actions: phashset![],
                stack_frames: Vec::new(),
            },
        );

        session
            .push_rpc_arg(PTC_SYNC_EVENT_INFO_ID, ptc_sync_event_info)
            .await?;
    }

    Ok(RpcInteractWithUnitRet::new())
}

pub async fn on_rpc_leave_cur_dungeon(
    session: &NetworkSession,
    _arg: &RpcLeaveCurDungeonArg,
) -> Result<RpcLeaveCurDungeonRet> {
    Box::pin(enter_main_city(session)).await?;
    Ok(RpcLeaveCurDungeonRet::new())
}

pub async fn on_ptc_player_operation(
    _session: &NetworkSession,
    _arg: &PtcPlayerOperationArg,
) -> Result<PtcPlayerOperationRet> {
    Ok(PtcPlayerOperationRet::new())
}

pub async fn on_rpc_save_pos_in_main_city(
    _session: &NetworkSession,
    _arg: &RpcSavePosInMainCityArg,
) -> Result<RpcSavePosInMainCityRet> {
    Ok(RpcSavePosInMainCityRet::new())
}

fn create_player(id: u64) -> PlayerInfo {
    let mut player = util::create_default_player(id);

    let pos_in_main_city = player.pos_in_main_city.as_mut().unwrap();
    pos_in_main_city.initial_pos_id.replace(2);
    pos_in_main_city.position.replace(Vector3f {
        x: 30.31,
        y: 0.58002,
        z: 11.18,
    });

    if CONFIGURATION.skip_tutorial {
        let beginner_procedure = player.beginner_procedure_info.as_mut().unwrap();
        beginner_procedure.procedure_info.replace(6);
        player.nick_name.replace(String::from("xeondev"));
        player.avatar_id.replace(2021);
    }

    player
}

pub async fn enter_main_city(session: &NetworkSession) -> Result<()> {
    let hall_scene_uid = session.context.dungeon_manager.get_default_scene_uid();

    session
        .push_rpc_arg(
            PTC_ENTER_SECTION_ID,
            session
                .context
                .dungeon_manager
                .enter_scene_section(hall_scene_uid, 2)
                .take(),
        )
        .await?;

    session
        .push_rpc_arg(
            PTC_SYNC_SCENE_UNIT_ID,
            session.context.scene_unit_manager.sync(hall_scene_uid, 2),
        )
        .await?;

    session
        .push_rpc_arg(
            PTC_ENTER_SCENE_ID,
            session
                .context
                .dungeon_manager
                .enter_main_city()?
                .send_changes(session)
                .await?,
        )
        .await
}

pub async fn on_rpc_enter_world(
    session: &NetworkSession,
    _arg: &RpcEnterWorldArg,
) -> Result<RpcEnterWorldRet> {
    {
        let account = session.ns_prop_mgr.account_info.read();

        let player_uid = *account.players.as_ref().unwrap().first().unwrap(); // get first id from list
        session.set_cur_player(PlayerUID::from(player_uid), create_player(player_uid))?;
    }

    let item_manager = &session.context.item_manager;
    item_manager.add_resource(501, 120);
    item_manager.add_resource(10, 228);
    item_manager.add_resource(100, 1337);

    for avatar_id in data::iter_avatar_config_collection()
        .filter(|c| c.camp != 0)
        .map(|c| c.id)
    {
        item_manager.unlock_avatar(avatar_id);
    }

    for unlock_id in data::iter_unlock_config_collection().map(|c| c.id) {
        session.context.unlock_manager.unlock(unlock_id);
    }

    session.context.dungeon_manager.create_hall(1);
    session.context.scene_unit_manager.add_scene_units(2);

    let quest_manager = session.context.quest_manager.clone();
    quest_manager.add_world_quest(QuestInfo::MainCity {
        id: 10020002,
        finished_count: 0,
        collection_uid: 0,
        progress: 0,
        parent_quest_id: 0,
        state: QuestState::InProgress,
        finish_condition_progress: phashmap![],
        progress_time: 2111012,
        sort_id: 1000,
        bound_npc_and_interact: phashmap![],
    });

    quest_manager.add_world_quest(QuestInfo::Hollow {
        id: 10010002,
        finished_count: 0,
        collection_uid: 3405096459205774,
        progress: 0,
        parent_quest_id: 0,
        state: QuestState::Ready,
        sort_id: 1001,
        statistics: phashmap![],
        statistics_ext: pdkhashmap![],
        acquired_hollow_challenge_reward: 0,
        progress_time: 0,
        finish_condition_progress: phashmap![],
        dungeon_uid: 0,
    });

    session.context.yorozuya_quest_manager.add_hollow_quest(
        102,
        HollowQuestType::SideQuest,
        10010002,
    );

    if CONFIGURATION.skip_tutorial {
        Box::pin(enter_main_city(session)).await?;
    } else {
        let fresh_scene_uid = session.context.dungeon_manager.create_fresh().take();
        session
            .push_rpc_arg(
                PTC_ENTER_SCENE_ID,
                session
                    .context
                    .dungeon_manager
                    .enter_scene(fresh_scene_uid)
                    .unwrap()
                    .take(),
            )
            .await?;
    }

    session
        .push_rpc_arg(
            PTC_SYNC_SCENE_TIME_ID,
            PtcSyncSceneTimeArg {
                timestamp: 3600 * 8 * 1000,
                last_timestamp: 0,
            },
        )
        .await?;

    Ok(RpcEnterWorldRet::new(
        session.ns_prop_mgr.serialize_player_info(),
    ))
}

pub async fn on_rpc_reenter_world(
    session: &NetworkSession,
    _arg: &RpcReenterWorldArg,
) -> Result<RpcReenterWorldRet> {
    tracing::warn!("OnRpcReenterWorld: world re-entrance is not implemented yet, kicking player!");

    session
        .push_rpc_arg(
            PTC_KICK_PLAYER_ID,
            PtcKickPlayerArg {
                reason_id: 2,
                reason_str: String::new(),
            },
        )
        .await?;

    Ok(RpcReenterWorldRet::error(ErrorCode::Fail, Vec::new()))
}
