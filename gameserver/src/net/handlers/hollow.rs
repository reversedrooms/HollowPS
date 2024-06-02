use itertools::Itertools;
use qwer::{phashmap, phashset, PropertyHashMap, PropertyHashSet};
use std::collections::HashMap;

use crate::data;

use super::*;

pub async fn on_rpc_hollow_move(
    session: &mut NetworkSession,
    arg: &RpcHollowMoveArg,
) -> Result<RpcHollowMoveRet> {
    tracing::info!("Hollow movement {:?}", &arg);

    let destination_pos = *arg.positions.last().unwrap();
    let scene_uid = session.ns_prop_mgr.player_info.read().scene_uid.unwrap();

    let (ptc_hollow_grid, ptc_sync_hollow_event) = session
        .context
        .hollow_grid_manager
        .move_to(destination_pos, scene_uid);

    session
        .send_rpc_arg(PTC_HOLLOW_GRID_ID, &ptc_hollow_grid)
        .await?;

    if let Some(ptc_sync_hollow_event) = ptc_sync_hollow_event {
        session
            .send_rpc_arg(PTC_SYNC_HOLLOW_EVENT_INFO_ID, &ptc_sync_hollow_event)
            .await?;
    }

    let pos = PtcPositionInHollowChangedArg {
        player_uid: session.player_uid().raw(),
        hollow_level: arg.hollow_level,
        position: destination_pos,
    };

    session
        .send_rpc_arg(PTC_POSITION_IN_HOLLOW_CHANGED_ID, &pos)
        .await?;

    Ok(RpcHollowMoveRet::new(
        arg.hollow_level,
        *arg.positions.last().unwrap(),
    ))
}

pub async fn on_rpc_end_battle(
    session: &NetworkSession,
    arg: &RpcEndBattleArg,
) -> Result<RpcEndBattleRet> {
    tracing::info!("RpcEndBattle: {:?}", &arg);

    let player_uid = session.player_uid().raw();
    let (sync_event, hollow_finished) = session.context.hollow_grid_manager.battle_finished();

    if !hollow_finished {
        session
            .send_rpc_arg(PTC_SYNC_HOLLOW_EVENT_INFO_ID, &sync_event)
            .await?;
    } else {
        let _ = *session
            .context
            .dungeon_manager
            .hollow_finished()
            .send_changes(session)
            .await?;

        let ptc_dungeon_quest_finished = PtcDungeonQuestFinishedArg {
            player_uid,
            quest_id: 1001000101,
            success: true,
            reward_items: phashmap![],
            statistics: phashmap![(QuestStatisticsType::ArrivedLevel, 1)],
        };

        session
            .send_rpc_arg(PTC_DUNGEON_QUEST_FINISHED_ID, &ptc_dungeon_quest_finished)
            .await?;
    }

    let ptc_enter_scene = session
        .context
        .dungeon_manager
        .leave_battle()
        .unwrap()
        .send_changes(session)
        .await?
        .clone();

    session
        .send_rpc_arg(
            PTC_SYNC_HOLLOW_GRID_MAPS_ID,
            &session.context.hollow_grid_manager.sync_hollow_maps(
                player_uid,
                session.context.dungeon_manager.get_cur_scene_uid(),
            ),
        )
        .await?;

    let ptc_position_in_hollow_changed = PtcPositionInHollowChangedArg {
        player_uid,
        hollow_level: 1,
        position: session
            .context
            .hollow_grid_manager
            .get_cur_position_in_hollow(),
    };

    session
        .send_rpc_arg(
            PTC_POSITION_IN_HOLLOW_CHANGED_ID,
            &ptc_position_in_hollow_changed,
        )
        .await?;

    session
        .send_rpc_arg(PTC_ENTER_SCENE_ID, &ptc_enter_scene)
        .await?;

    Ok(RpcEndBattleRet::new(
        session
            .context
            .hollow_grid_manager
            .get_cur_event_template_id(),
        HashMap::new(),
    ))
}

pub async fn on_rpc_run_hollow_event_graph(
    session: &mut NetworkSession,
    arg: &RpcRunHollowEventGraphArg,
) -> Result<RpcRunHollowEventGraphRet> {
    tracing::info!("Run hollow event graph {:?}", arg);

    let scene_uid = session.ns_prop_mgr.player_info.read().scene_uid.unwrap();

    if arg.event_graph_uid == 3405096459205834 {
        // Perform (cutscene)
        let finish_perform = PtcSyncHollowEventInfoArg {
            event_graph_uid: 3405096459205834,
            hollow_event_template_id: 1000108,
            event_graph_id: 1000108,
            updated_event: EventInfo {
                id: 1000,
                cur_action_id: -1,
                action_move_path: vec![1001, 1002, -1],
                state: EventState::Finished,
                prev_state: EventState::Running,
                cur_action_info: ActionInfo::None {},
                cur_action_state: ActionState::Init,
                predicated_failed_actions: phashset![],
                stack_frames: Vec::new(),
            },
            specials: phashmap![],
        };
        session
            .send_rpc_arg(PTC_SYNC_HOLLOW_EVENT_INFO_ID, &finish_perform)
            .await?;

        let (ptc_hollow_grid, ptc_sync_hollow_event) =
            session.context.hollow_grid_manager.move_to(22, scene_uid);

        session
            .send_rpc_arg(PTC_HOLLOW_GRID_ID, &ptc_hollow_grid)
            .await?;
        if let Some(ptc_sync_hollow_event) = ptc_sync_hollow_event {
            session
                .send_rpc_arg(PTC_SYNC_HOLLOW_EVENT_INFO_ID, &ptc_sync_hollow_event)
                .await?;
        }
    } else {
        let (sync_hollow_event, hollow_grid, trigger_battle_id, hollow_finished) = session
            .context
            .hollow_grid_manager
            .run_event_graph(arg.event_graph_uid, arg.event_id, arg.move_path.clone());

        if !hollow_finished {
            session
                .send_rpc_arg(PTC_SYNC_HOLLOW_EVENT_INFO_ID, &sync_hollow_event)
                .await?;
        }
        session
            .send_rpc_arg(PTC_HOLLOW_GRID_ID, &hollow_grid)
            .await?;

        if hollow_finished {
            let _ = *session
                .context
                .dungeon_manager
                .hollow_finished()
                .send_changes(session)
                .await?;

            let ptc_dungeon_quest_finished = PtcDungeonQuestFinishedArg {
                player_uid: session.player_uid().raw(),
                quest_id: 1001000101,
                success: true,
                reward_items: phashmap![],
                statistics: phashmap![],
            };

            session
                .send_rpc_arg(PTC_DUNGEON_QUEST_FINISHED_ID, &ptc_dungeon_quest_finished)
                .await?;
        }

        if let Some(trigger_battle_id) = trigger_battle_id {
            let hollow_uid = *session
                .ns_prop_mgr
                .player_info
                .read()
                .scene_uid
                .as_ref()
                .unwrap();
            let battle_scene_uid = *session
                .context
                .dungeon_manager
                .create_fight(trigger_battle_id, hollow_uid)
                .send_changes(session)
                .await?;

            let ptc_position_in_hollow_changed = PtcPositionInHollowChangedArg {
                player_uid: session.player_uid().raw(),
                hollow_level: 1,
                position: session
                    .context
                    .hollow_grid_manager
                    .get_cur_position_in_hollow(),
            };

            session
                .send_rpc_arg(
                    PTC_POSITION_IN_HOLLOW_CHANGED_ID,
                    &ptc_position_in_hollow_changed,
                )
                .await?;

            session
                .send_rpc_arg(
                    PTC_ENTER_SCENE_ID,
                    session
                        .context
                        .dungeon_manager
                        .enter_battle(battle_scene_uid)
                        .send_changes(session)
                        .await?,
                )
                .await?;
        }
    }

    Ok(RpcRunHollowEventGraphRet::new())
}

pub async fn on_rpc_start_hollow_quest(
    session: &NetworkSession,
    arg: &RpcStartHollowQuestArg,
) -> Result<RpcStartHollowQuestRet> {
    tracing::info!("start hollow quest: {arg:?}");

    // Set avatar HP properties
    for (_idx, avatar_uid) in &arg.avatar_map {
        let update_properties = {
            let player_info = session.ns_prop_mgr.player_info.read();
            let items = player_info.items.as_ref().unwrap();
            let Some(ItemInfo::Avatar { id, .. }) = items
                .iter()
                .find(|(uid, _)| **uid == *avatar_uid)
                .map(|(_, item)| item)
            else {
                return Ok(RpcStartHollowQuestRet::error(
                    ErrorCode::ObjectNotExist,
                    Vec::new(),
                ));
            };

            let avatar_config = data::iter_avatar_config_collection()
                .find(|c| c.id == *id)
                .unwrap();

            PtcPropertyChangedArg {
                scene_unit_uid: *avatar_uid,
                is_partial: true,
                changed_properties: phashmap![(1, avatar_config.hp), (111, avatar_config.hp)],
            }
        };

        session
            .send_rpc_arg(PTC_PROPERTY_CHANGED_ID, &update_properties)
            .await?;
    }

    let avatars = arg
        .avatar_map
        .iter()
        .sorted_by_key(|kv| kv.0)
        .map(|(_idx, uid)| *uid)
        .collect::<Vec<_>>();
    let (dungeon_uid, scene_uid) = *session
        .context
        .dungeon_manager
        .create_hollow(10001, 10010001, &avatars)
        .send_changes(session)
        .await?;

    session
        .context
        .quest_manager
        .add_quest_to_collection(
            dungeon_uid,
            QuestInfo::DungeonInner {
                id: 1001000101,
                finished_count: 0,
                collection_uid: 0,
                progress: 0,
                parent_quest_id: 10010001,
                state: QuestState::InProgress,
                finish_condition_progress: phashmap![],
                progress_time: 2111605,
                sort_id: 2000,
            },
        )
        .send_changes(session)
        .await?;

    let ptc_enter_scene = session
        .context
        .dungeon_manager
        .enter_scene(scene_uid)?
        .send_changes(session)
        .await?
        .clone();

    session.context.hollow_grid_manager.init_default_map();

    session
        .send_rpc_arg(
            PTC_SYNC_HOLLOW_GRID_MAPS_ID,
            &session
                .context
                .hollow_grid_manager
                .sync_hollow_maps(session.player_uid().raw(), scene_uid),
        )
        .await?;

    let ptc_position_in_hollow_changed = PtcPositionInHollowChangedArg {
        player_uid: session.player_uid().raw(),
        hollow_level: 1,
        position: session
            .context
            .hollow_grid_manager
            .get_cur_position_in_hollow(),
    };

    session
        .send_rpc_arg(
            PTC_POSITION_IN_HOLLOW_CHANGED_ID,
            &ptc_position_in_hollow_changed,
        )
        .await?;

    let ptc_sync_hollow_event_info = PtcSyncHollowEventInfoArg {
        event_graph_uid: 3405096459205834,
        hollow_event_template_id: 1000108,
        event_graph_id: 1000108,
        updated_event: EventInfo {
            id: 1000,
            cur_action_id: 1001,
            action_move_path: vec![1001],
            state: EventState::WaitingClient,
            prev_state: EventState::Running,
            cur_action_info: ActionInfo::None {},
            cur_action_state: ActionState::Init,
            predicated_failed_actions: phashset![],
            stack_frames: Vec::new(),
        },
        specials: phashmap![],
    };

    session
        .send_rpc_arg(PTC_SYNC_HOLLOW_EVENT_INFO_ID, &ptc_sync_hollow_event_info)
        .await?;

    session
        .send_rpc_arg(PTC_ENTER_SCENE_ID, &ptc_enter_scene)
        .await?;
    Ok(RpcStartHollowQuestRet::new())
}
