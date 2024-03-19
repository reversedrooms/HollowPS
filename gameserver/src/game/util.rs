use protocol::*;
use qwer::{
    pdkhashmap, phashmap, phashset, PropertyDoubleKeyHashMap, PropertyHashMap, PropertyHashSet,
};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn cur_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub fn create_default_account(id: u64) -> AccountInfo {
    AccountInfo {
        account_name: Some(format!("1_{id}")),
        players: Some(vec![id]),
        gm_level: Some(10),
        account_type: Some(1),
        register_cps: Some(String::new()),
    }
}

pub fn create_default_player(id: u64) -> PlayerInfo {
    PlayerInfo {
        uid: Some(id),
        account_name: Some(format!("1_{id}")),
        items: Some(qwer::phashmap![(
            3405096459205780,
            ItemInfo::Buddy {
                uid: 3405096459205780,
                id: 50012,
                count: 1,
                package: 3,
                first_get_time: 0,
            }
        )]),
        dungeon_collection: Some(DungeonCollection {
            dungeons: Some(qwer::phashmap![]),
            scenes: Some(qwer::phashmap![]),
            default_scene_uid: Some(0),
            transform: Some(Transform::default()),
            used_story_mode: Some(true),
            used_manual_qte_mode: Some(true),
        }),
        properties: Some(pdkhashmap![]),
        scene_properties: Some(pdkhashmap![]),
        quest_data: Some(QuestData {
            quests: Some(pdkhashmap![]),
            is_afk: Some(false),
            unlock_condition_progress: Some(pdkhashmap![]),
            world_quest_collection_uid: Some(0),
            world_quest_for_cur_dungeon: Some(0),
            world_quest_for_cur_dungeon_afk: Some(0),
        }),
        joined_chat_rooms: Some(Vec::new()),
        last_enter_world_timestamp: Some(0),
        scene_uid: Some(0),
        archive_info: Some(ArchiveInfo {
            videotapes_info: Some(phashmap![]),
        }),
        auto_recovery_info: Some(phashmap![(
            501,
            AutoRecoveryInfo {
                buy_times: 0,
                last_recovery_timestamp: 0,
            }
        )]),
        unlock_info: Some(UnlockInfo {
            condition_progress: Some(pdkhashmap![]),
            unlocked_list: Some(phashset![]),
        }),
        yorozuya_info: Some(YorozuyaInfo {
            yorozuya_level: Some(1),
            yorozuya_rank: Some(1),
            gm_enabled: Some(true),
            gm_quests: Some(phashmap![]),
            finished_hollow_quest_count: Some(0),
            finished_hollow_quest_count_of_type: Some(phashmap![]),
            hollow_quests: Some(pdkhashmap![]),
            urgent_quests_queue: Some(phashmap![]),
            unlock_hollow_id: Some(vec![102]),
            unlock_hollow_id_progress: Some(pdkhashmap![]),
            last_refresh_timestamp_common: Some(0),
            last_refresh_timestamp_urgent: Some(0),
            next_refresh_timestamp_urgent: Some(0),
        }),
        equip_gacha_info: Some(EquipGachaInfo {
            avatar_level_advance_times: Some(0),
            equip_star_up_times: Some(0),
            security_num_by_lv: Some(phashmap![]),
            smithy_level: Some(0),
            total_gacha_times: Some(0),
        }),
        beginner_procedure_info: Some(BeginnerProcedureInfo {
            procedure_info: Some(0),
        }),
        pos_in_main_city: Some(PlayerPosInMainCity {
            position: Some(Vector3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            rotation: Some(Vector3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            initial_pos_id: Some(0),
        }),
        fairy_info: Some(FairyInfo {
            condition_progress: Some(pdkhashmap![]),
            fairy_groups: Some(phashmap![]),
        }),
        popup_window_info: Some(PopupWindowInfo {
            condition_progress: Some(pdkhashmap![]),
            popup_window_list: Some(Vec::new()),
        }),
        tips_info: Some(TipsInfo {
            tips_list: Some(Vec::new()),
            tips_group: Some(Vec::new()),
            tips_condition_progress: Some(pdkhashmap![]),
            tips_group_condition_progress: Some(pdkhashmap![]),
        }),
        main_city_quest_data: Some(MainCityQuestData {
            in_progress_quests: Some(Vec::new()),
            exicing_finish_script_group: Some(vec![10020001]),
        }),
        embattles: Some(Embattles {
            last_embattles: Some(phashmap![]),
        }),
        day_change_info: Some(DayChangeInfo {
            last_daily_refresh_timing: Some(0),
        }),
        npcs_info: Some(PlayerNPCsInfo {
            npcs_info: Some(phashmap![]),
            destroy_npc_when_leave_section: Some(phashset![]),
        }),
        scripts_to_execute: Some(pdkhashmap![]),
        scripts_to_remove: Some(phashmap![]),
        last_leave_world_timestamp: Some(0),
        muip_data: Some(MUIPData {
            alread_cmd_uids: Some(phashset![]),
            ban_end_time: Some(String::new()),
            tag_value: Some(0),
            scene_pass_times: Some(phashmap![]),
            scene_enter_times: Some(phashmap![]),
            dungeon_pass_times: Some(phashmap![]),
            dungeon_enter_times: Some(phashmap![]),
            ban_begin_time: Some(String::new()),
            game_total_time: Some(0),
            language_type: Some(0),
        }),
        nick_name: Some(String::new()),
        ramen_data: Some(RamenData {
            unlock_ramen: Some(phashset![20301, 20401, 20501, 20601, 20201]),
            cur_ramen: Some(0),
            used_times: Some(0),
            unlock_initiative_item: Some(phashset![]),
            unlock_ramen_condition_progress: Some(pdkhashmap![]),
            unlock_item_condition_progress: Some(pdkhashmap![]),
            has_mystical_spice: Some(true),
            unlock_has_mystical_spice_condition_progress: Some(phashmap![]),
            cur_mystical_spice: Some(0),
            unlock_mystical_spice: Some(phashset![
                30101, 30601, 30201, 30501, 30301, 30801, 31201, 30401, 31401, 31001
            ]),
            unlock_mystical_spice_condition_progress: Some(pdkhashmap![]),
            unlock_initiative_item_group: Some(phashset![]),
            hollow_item_history: Some(phashmap![]),
            initial_item_ability: Some(0),
            new_unlock_ramen: Some(Vec::new()),
            eat_ramen_times: Some(0),
            make_hollow_item_times: Some(0),
            new_unlock_initiative_item: Some(phashset![]),
        }),

        shop: Some(ShopsInfo {
            shops: Some(phashmap![]),
            shop_buy_times: Some(0),
            vip_level: Some(0),
        }),
        vhs_store_data: Some(VHSStoreData {
            store_level: Some(0),
            unreceived_reward: Some(0),
            hollow_enter_times: Some(0),
            last_receive_time: Some(0),
            vhs_collection_slot: Some(Vec::new()),
            unlock_vhs_collection: Some(phashset![]),
            already_trending: Some(phashset![]),
            unlock_trending_condition_progress: Some(pdkhashmap![]),
            is_need_refresh: Some(true),
            scripts_id: Some(phashset![]),
            store_exp: Some(0),
            is_level_chg_tips: Some(true),
            vhs_hollow: Some(Vec::new()),
            is_receive_trending_reward: Some(false),
            is_need_first_trending: Some(false),
            last_basic_script: Some(0),
            is_complete_first_trending: Some(false),
            last_basic_npc: Some(0),
            can_random_trending: Some(phashset![]),
            vhs_trending_info: Some(Vec::new()),
            unlock_vhs_trending_info: Some(phashmap![]),
            vhs_flow: Some(0),
            received_reward: Some(0),
            last_reward: Some(0),
            last_exp: Some(0),
            last_flow: Some(0),
            last_vhs_trending_info: Some(Vec::new()),
            new_know_trend: Some(Vec::new()),
            quest_finish_script: Some(pdkhashmap![]),
            quest_finish_scripts_id: Some(phashset![]),
            total_received_reward: Some(phashmap![]),
            last_vhs_npc_info: Some(Vec::new()),
            vhs_npc_info: Some(Vec::new()),
            npc_info: Some(phashset![]),
            total_received_reward_times: Some(0),
        }),
        operation_mail_receive_info: Some(OperationMailReceiveInfo {
            receive_list: Some(phashset![]),
            condition_progress: Some(pdkhashmap![]),
        }),
        second_last_enter_world_timestamp: Some(0),
        login_times: Some(1),
        create_timestamp: Some(cur_timestamp_ms()),
        gender: Some(0),
        avatar_id: Some(0),
        prev_scene_uid: Some(2),
        register_cps: Some(String::new()),
        register_platform: Some(3),
        pay_info: Some(PayInfo {
            month_total_pay: Some(0),
        }),
        private_npcs: Some(phashmap![]),
        battle_event_info: Some(BattleEventInfo {
            unlock_battle: Some(phashset![]),
            unlock_battle_condition_progress: Some(pdkhashmap![]),
            alread_rand_battle: Some(pdkhashmap![]),
            alread_battle_stage: Some(Vec::new()),
            rand_battle_type: Some(phashmap![]),
        }),
        gm_data: Some(GMData {
            register_conditions: Some(phashset![]),
            condition_proress: Some(pdkhashmap![]),
            completed_conditions: Some(phashset![]),
        }),
        player_mail_ext_infos: Some(PlayerMailExtInfos {
            player_mail_ext_info: Some(phashmap![]),
        }),
        single_dungeon_group: Some(SingleDungeonGroup {
            dungeons: Some(phashmap![]),
            scenes: Some(pdkhashmap![]),
            npcs: Some(pdkhashmap![]),
            section: Some(pdkhashmap![]),
        }),
        newbie_info: Some(NewbieInfo {
            unlocked_id: Some(phashset![3]),
            condition_progress: Some(pdkhashmap![]),
        }),
        loading_page_tips_info: Some(LoadingPageTipsInfo {
            unlocked_id: Some(phashset![1, 2, 3]),
            condition_progress: Some(pdkhashmap![]),
        }),
        switch_of_story_mode: Some(true),
        switch_of_qte: Some(true),
        collect_map: Some(CollectMap {
            card_map: Some(phashset![]),
            curse_map: Some(phashset![]),
            unlock_cards: Some(phashset![]),
            unlock_curses: Some(phashset![]),
            event_icon_map: Some(phashset![]),
            unlock_events: Some(phashset![]),
            new_card_map: Some(phashset![]),
            new_curse_map: Some(phashset![]),
            new_event_icon_map: Some(phashset![]),
            unlock_event_icon_condition_progress: Some(pdkhashmap![]),
            unlock_card_condition_progress: Some(pdkhashmap![]),
            unlock_curse_condition_progress: Some(pdkhashmap![]),
            unlock_event_condition_progress: Some(pdkhashmap![]),
            unlock_event_icons: Some(phashset![]),
        }),
        areas_info: Some(AreasInfo {
            area_owners_info: Some(pdkhashmap![]),
            sequence: Some(0),
        }),
        bgm_info: Some(BGMInfo { bgm_id: Some(1) }),
        main_city_objects_state: Some(phashmap![]),
        hollow_info: Some(HollowInfo {
            banned_hollow_event: Some(phashset![]),
        }),
    }
}
