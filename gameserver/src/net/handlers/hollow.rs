use qwer::{phashmap, phashset, PropertyHashMap, PropertyHashSet};
use std::collections::HashMap;

use super::*;

pub async fn on_rpc_hollow_move_arg(
    session: &mut NetworkSession,
    arg: &RpcHollowMoveArg,
) -> Result<()> {
    tracing::info!("Hollow movement {:?}", &arg);

    let destination_pos = *arg.positions.last().unwrap();
    let scene_uid = session.get_player().scene_uid.unwrap();

    let hollow_grid_manager = session.context.hollow_grid_manager.borrow();
    let (ptc_hollow_grid, ptc_sync_hollow_event) =
        hollow_grid_manager.move_to(destination_pos, scene_uid);

    session.send_rpc_arg(114, &ptc_hollow_grid).await?;
    if let Some(ptc_sync_hollow_event) = ptc_sync_hollow_event {
        session.send_rpc_arg(210, &ptc_sync_hollow_event).await?;
    }

    let pos = PtcPositionInHollowChangedArg {
        player_uid: 1337,
        hollow_level: arg.hollow_level,
        position: destination_pos,
    };

    session.send_rpc_arg(141, &pos).await?;

    session
        .send_rpc_ret(RpcHollowMoveRet::new(
            arg.hollow_level,
            *arg.positions.last().unwrap(),
        ))
        .await
}

pub async fn on_rpc_end_battle_arg(session: &NetworkSession, arg: &RpcEndBattleArg) -> Result<()> {
    tracing::info!("RpcEndBattle: {:?}", &arg);

    let player_uid = session.get_player_uid();
    let hollow_grid_manager = session.context.hollow_grid_manager.borrow();

    let (sync_event, hollow_finished) = hollow_grid_manager.battle_finished();

    if !hollow_finished {
        session.send_rpc_arg(210, &sync_event).await?;
    } else {
        let dungeon_manager = session.context.dungeon_manager.borrow();
        let cur_scene = *dungeon_manager
            .hollow_finished()
            .send_changes(session)
            .await?;

        let ptc_dungeon_quest_finished = PtcDungeonQuestFinishedArg {
            player_uid: 1337,
            quest_id: 1001000101,
            success: true,
            reward_items: phashmap![],
            statistics: phashmap![],
        };

        session
            .send_rpc_arg(148, &ptc_dungeon_quest_finished)
            .await?;
    }

    let dungeon_manager = session.context.dungeon_manager.borrow();
    let ptc_enter_scene = dungeon_manager
        .leave_battle()
        .unwrap()
        .send_changes(session)
        .await?
        .clone();

    session
        .send_rpc_arg(
            124,
            &hollow_grid_manager.sync_hollow_maps(player_uid, dungeon_manager.get_cur_scene_uid()),
        )
        .await?;

    let ptc_position_in_hollow_changed = PtcPositionInHollowChangedArg {
        player_uid,
        hollow_level: 1,
        position: hollow_grid_manager.get_cur_position_in_hollow(),
    };

    session
        .send_rpc_arg(141, &ptc_position_in_hollow_changed)
        .await?;

    session.send_rpc_arg(118, &ptc_enter_scene).await?;

    session
        .send_rpc_ret(RpcEndBattleRet::new(
            hollow_grid_manager.get_cur_event_template_id(),
            HashMap::new(),
        ))
        .await
}

pub async fn on_rpc_run_hollow_event_graph_arg(
    session: &mut NetworkSession,
    arg: &RpcRunHollowEventGraphArg,
) -> Result<()> {
    tracing::info!("Run hollow event graph {:?}", arg);

    let scene_uid = session.get_player().scene_uid.unwrap();

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
        session.send_rpc_arg(210, &finish_perform).await?;

        let hollow_grid_manager = session.context.hollow_grid_manager.borrow();
        let (ptc_hollow_grid, ptc_sync_hollow_event) = hollow_grid_manager.move_to(22, scene_uid);

        session.send_rpc_arg(114, &ptc_hollow_grid).await?;
        if let Some(ptc_sync_hollow_event) = ptc_sync_hollow_event {
            session.send_rpc_arg(210, &ptc_sync_hollow_event).await?;
        }
    } else {
        let hollow_grid_manager = session.context.hollow_grid_manager.borrow();
        let (sync_hollow_event, hollow_grid, trigger_battle_id, hollow_finished) =
            hollow_grid_manager.run_event_graph(
                arg.event_graph_uid,
                arg.event_id,
                arg.move_path.clone(),
            );

        if !hollow_finished {
            session.send_rpc_arg(210, &sync_hollow_event).await?;
        }
        session.send_rpc_arg(114, &hollow_grid).await?;

        if hollow_finished {
            let dungeon_manager = session.context.dungeon_manager.borrow();
            let cur_scene = *dungeon_manager
                .hollow_finished()
                .send_changes(session)
                .await?;

            let ptc_dungeon_quest_finished = PtcDungeonQuestFinishedArg {
                player_uid: 1337,
                quest_id: 1001000101,
                success: true,
                reward_items: phashmap![],
                statistics: phashmap![],
            };

            session
                .send_rpc_arg(148, &ptc_dungeon_quest_finished)
                .await?;
        }

        if let Some(trigger_battle_id) = trigger_battle_id {
            let dungeon_manager = session.context.dungeon_manager.borrow();
            let hollow_uid = *session.get_player().scene_uid.as_ref().unwrap();
            let battle_scene_uid = *dungeon_manager
                .create_fight(trigger_battle_id, hollow_uid)
                .send_changes(session)
                .await?;

            let ptc_position_in_hollow_changed = PtcPositionInHollowChangedArg {
                player_uid: 1337,
                hollow_level: 1,
                position: hollow_grid_manager.get_cur_position_in_hollow(),
            };

            session
                .send_rpc_arg(141, &ptc_position_in_hollow_changed)
                .await?;

            session
                .send_rpc_arg(
                    118,
                    dungeon_manager
                        .enter_battle(battle_scene_uid)
                        .send_changes(session)
                        .await?,
                )
                .await?;
        }
    }

    session.send_rpc_ret(RpcRunHollowEventGraphRet::new()).await
}

pub async fn on_rpc_start_hollow_quest_arg(
    session: &NetworkSession,
    arg: &RpcStartHollowQuestArg,
) -> Result<()> {
    tracing::info!("start hollow quest: {arg:?}");

    for (_idx, avatar_uid) in &arg.avatar_map {
        // Set character HP
        let update_properties = PtcPropertyChangedArg {
            scene_unit_uid: *avatar_uid,
            is_partial: true,
            changed_properties: phashmap![(1, 500), (111, 500)],
        };

        session.send_rpc_arg(129, &update_properties).await?;
    }

    let dungeon_manager = session.context.dungeon_manager.borrow();

    let avatars = arg
        .avatar_map
        .iter()
        .map(|(_idx, uid)| *uid)
        .collect::<Vec<_>>();
    let (dungeon_uid, scene_uid) = *dungeon_manager
        .create_hollow(10001, 10010001, &avatars)
        .send_changes(session)
        .await?;

    let quest_manager = session.context.quest_manager.borrow();
    quest_manager
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

    let ptc_enter_scene = dungeon_manager
        .enter_scene(scene_uid)?
        .send_changes(session)
        .await?
        .clone();

    let hollow_grid_manager = session.context.hollow_grid_manager.borrow();
    hollow_grid_manager.init_default_map();

    session
        .send_rpc_arg(
            124,
            &hollow_grid_manager.sync_hollow_maps(session.get_player_uid(), scene_uid),
        )
        .await?;

    let ptc_position_in_hollow_changed = PtcPositionInHollowChangedArg {
        player_uid: 1337,
        hollow_level: 1,
        position: hollow_grid_manager.get_cur_position_in_hollow(),
    };

    session
        .send_rpc_arg(141, &ptc_position_in_hollow_changed)
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
        .send_rpc_arg(210, &ptc_sync_hollow_event_info)
        .await?;

    session.send_rpc_arg(118, &ptc_enter_scene).await?;
    session.send_rpc_ret(RpcStartHollowQuestRet::new()).await
}
