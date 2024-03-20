use qwer::{
    pdkhashmap, phashmap, phashset, PropertyDoubleKeyHashMap, PropertyHashMap, PropertyHashSet,
};

use crate::config;
use crate::game::{globals, util};

use super::*;

static UNLOCK_AVATARS: [i32; 12] = [
    1011, 1021, 1031, 1041, 1061, 1081, 1091, 1101, 1111, 1121, 1131, 1141,
];

static UNLOCK_FEATURES: [i32; 35] = [
    1001, 1002, 1003, 1004, 1005, 1006, 1007, 1008, 1009, 1010, 1011, 1013, 1014, 1015, 1016, 1017,
    1018, 1019, 10001, 10002, 10003, 10004, 10005, 10006, 10007, 10008, 10009, 10010, 10012, 10013,
    10014, 10015, 10017, 10018, 10019,
];

pub async fn on_rpc_run_event_graph_arg(
    session: &NetworkSession,
    arg: &RpcRunEventGraphArg,
) -> Result<()> {
    tracing::info!("RunEventGraph requested");

    let scene_unit_mgr = session.context.scene_unit_manager.borrow();
    let unit = scene_unit_mgr.get(arg.owner_uid);

    let SceneUnitProtocolInfo::NpcProtocolInfo { tag, id, .. } = unit;
    let main_city_object = config::get_main_city_object(tag, id).unwrap();

    let mut ptc_sync_event_info = PtcSyncEventInfoArg {
        owner_type: EventGraphOwnerType::SceneUnit,
        owner_uid: arg.owner_uid,
        updated_events: pdkhashmap![],
    };

    ptc_sync_event_info.updated_events.insert(
        *main_city_object.default_interact_ids.first().unwrap(),
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

    session.send_rpc_arg(177, &ptc_sync_event_info).await?;
    session.send_rpc_ret(RpcRunEventGraphRet::new()).await
}

pub async fn on_rpc_finish_event_graph_perform_show_arg(
    session: &NetworkSession,
    arg: &RpcFinishEventGraphPerformShowArg,
) -> Result<()> {
    tracing::info!("FinishEventGraphPerformShow");

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

    session.send_rpc_arg(177, &ptc_sync_event_info).await?;
    session
        .send_rpc_ret(RpcFinishEventGraphPerformShowRet::new())
        .await
}

pub async fn on_rpc_interact_with_unit_arg(
    session: &NetworkSession,
    arg: &RpcInteractWithUnitArg,
) -> Result<()> {
    tracing::info!("InteractWithUnit");

    let scene_unit_mgr = session.context.scene_unit_manager.borrow();
    let unit = scene_unit_mgr.get(arg.unit_uid);

    let SceneUnitProtocolInfo::NpcProtocolInfo { tag, id, .. } = unit;
    let main_city_object = config::get_main_city_object(tag, id).unwrap();

    let mut ptc_sync_event_info = PtcSyncEventInfoArg {
        owner_type: EventGraphOwnerType::SceneUnit,
        owner_uid: arg.unit_uid,
        updated_events: pdkhashmap![],
    };

    ptc_sync_event_info.updated_events.insert(
        *main_city_object.default_interact_ids.first().unwrap(),
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

    session.send_rpc_arg(177, &ptc_sync_event_info).await?;
    session.send_rpc_ret(RpcInteractWithUnitRet::new()).await
}

pub async fn on_rpc_leave_cur_dungeon_arg(
    session: &NetworkSession,
    _arg: &RpcLeaveCurDungeonArg,
) -> Result<()> {
    Box::pin(enter_main_city(session)).await?;
    session.send_rpc_ret(RpcLeaveCurDungeonRet::new()).await
}

pub async fn on_ptc_player_operation_arg(
    session: &NetworkSession,
    _arg: &PtcPlayerOperationArg,
) -> Result<()> {
    session.send_rpc_ret(PtcPlayerOperationRet::new()).await
}

pub async fn on_rpc_save_pos_in_main_city_arg(
    session: &NetworkSession,
    arg: &RpcSavePosInMainCityArg,
) -> Result<()> {
    tracing::info!("MainCity pos updated");

    session.send_rpc_ret(RpcSavePosInMainCityRet::new()).await
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

    if globals::should_skip_tutorial() {
        let beginner_procedure = player.beginner_procedure_info.as_mut().unwrap();
        beginner_procedure.procedure_info.replace(6);
        player.nick_name.replace(String::from("xeondev"));
        player.avatar_id.replace(2021);
    }

    player
}

pub async fn enter_main_city(session: &NetworkSession) -> Result<()> {
    let dungeon_manager = session.context.dungeon_manager.borrow();
    let scene_unit_mgr = session.context.scene_unit_manager.borrow();

    let hall_scene_uid = dungeon_manager.get_default_scene_uid();

    session
        .send_rpc_arg(
            243,
            dungeon_manager
                .enter_scene_section(hall_scene_uid, 2)
                .unwrap(),
        )
        .await?;

    session
        .send_rpc_arg(180, &scene_unit_mgr.sync(hall_scene_uid, 2))
        .await?;

    session
        .send_rpc_arg(
            118,
            dungeon_manager
                .enter_main_city()?
                .send_changes(session)
                .await?,
        )
        .await
}

pub async fn on_rpc_enter_world_arg(
    session: &NetworkSession,
    _arg: &RpcEnterWorldArg,
) -> Result<()> {
    let account = session.get_account();

    let id = *account.players.as_ref().unwrap().first().unwrap(); // get first id from list
    *session.get_player_mut() = create_player(id);

    let item_manager = session.context.item_manager.borrow();

    item_manager.add_resource(501, 120);
    item_manager.add_resource(10, 228);
    item_manager.add_resource(100, 1337);

    for avatar_id in UNLOCK_AVATARS {
        item_manager.unlock_avatar(avatar_id);
    }

    let unlock_manager = session.context.unlock_manager.borrow();
    for unlock_id in UNLOCK_FEATURES {
        unlock_manager.unlock(unlock_id);
    }

    let dungeon_manager = session.context.dungeon_manager.borrow();
    dungeon_manager.create_hall(1);
    let scene_unit_mgr = session.context.scene_unit_manager.borrow();
    scene_unit_mgr.add_default_units();

    let quest_manager = session.context.quest_manager.borrow();
    quest_manager.add_world_quest(QuestInfo::MainCity {
        id: 10020001,
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

    let yorozuya_quest_manager = session.context.yorozuya_quest_manager.borrow();
    yorozuya_quest_manager.add_hollow_quest(102, HollowQuestType::SideQuest, 10010002);

    if globals::should_skip_tutorial() {
        Box::pin(enter_main_city(session)).await?;
    } else {
        let fresh_scene_uid = *dungeon_manager.create_fresh().unwrap();
        session
            .send_rpc_arg(
                118,
                dungeon_manager
                    .enter_scene(fresh_scene_uid)
                    .unwrap()
                    .unwrap(),
            )
            .await?;
    }

    session
        .send_rpc_ret(RpcEnterWorldRet::new(
            session.ns_prop_mgr.serialize_player_info(),
        ))
        .await
}
