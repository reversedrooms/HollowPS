use super::*;

#[derive(Debug, Default)]
pub struct PropertyBlob {
    pub stream: Vec<u8>,
}

impl std::fmt::Display for PropertyBlob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.stream)
    }
}

impl OctData for PropertyBlob {
    fn marshal_to<W: std::io::Write>(
        &self,
        w: &mut W,
        bt_property_tag: u16,
    ) -> Result<(), std::io::Error> {
        if self.stream.is_empty() {
            (0i32).marshal_to(w, bt_property_tag)?;
            return Ok(());
        }
        (self.stream.len() as i32).marshal_to(w, bt_property_tag)?;
        w.write_all(&self.stream)?;
        Ok(())
    }

    fn unmarshal_from<R: std::io::Read>(
        r: &mut R,
        bt_property_tag: u16,
    ) -> Result<Self, std::io::Error> {
        let len = i32::unmarshal_from(r, bt_property_tag)?;
        assert!(len >= 0, "PropertyBlob stream length can't be negative");

        let mut stream = vec![0; len as usize];
        r.read_exact(&mut stream)?;
        Ok(Self { stream })
    }
}

#[derive(OctData, Copy, Clone, Debug, Default)]
pub struct Vector3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(OctData, Clone, Debug, Default)]
pub struct Transform {
    pub position: Vector3f,
    pub rotation: Vector3f,
}

#[derive(OctData, Debug)]
pub struct FightDropInfo {
    pub drop_pack_id: i32,
    pub param_1: i32,
}

#[derive(OctData, Debug)]
pub struct ChallengeResultInfo {
    pub param_1: i32,
}

#[derive(OctData, Debug)]
pub struct ItemIDCount {
    pub id: i32,
    pub count: i32,
}

#[derive(OctData, Clone, Default, Debug, PartialEq, Eq)]
#[property_object(u16, 0x01)]
#[root]
pub struct AccountInfo {
    #[tag = 1]
    pub account_name: Option<String>,
    #[tag = 2]
    #[property_object(u8, 0x01)]
    pub players: Option<Vec<u64>>,
    #[tag = 3]
    pub gm_level: Option<u8>,
    #[tag = 4]
    pub account_type: Option<i32>,
    #[tag = 5]
    pub register_cps: Option<String>,
}

#[derive(OctData, Clone, Debug)]
pub struct TimeEventInfo {
    pub executed_count: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct TimeEventGroupInfo {
    pub group_id: i32,
    pub executing_scripts: PropertyHashSet<i32>,
    pub complete_time: u64,
    pub time_events_info: PropertyHashMap<i32, TimeEventInfo>,
    pub bound_npc_and_interact: PropertyHashMap<u64, BoundNPCAndInteractInfo>,
    pub executing_time_event: PropertyHashSet<i32>,
}

#[derive(OctData, Clone, Debug)]
pub struct MainCityTimeInfo {
    pub initial_time: u32,
    pub passed_milliseconds: u64,
    pub executing_event_groups: PropertyHashSet<i32>,
    pub unlocked_time_events: PropertyHashSet<i32>,
    #[skip_property]
    pub time_event_groups_info: PropertyHashMap<i32, TimeEventGroupInfo>,
    #[skip_property]
    pub condition_progress_of_unlock: PropertyDoubleKeyHashMap<i32, i32, i32>,
    #[skip_property]
    pub condition_progress_of_end: PropertyDoubleKeyHashMap<i32, i32, i32>,
    pub ended_time_events: PropertyHashSet<i32>,
    pub leave_time: u64,
}

#[derive(OctData, Clone, Debug)]
pub struct AvatarPropertyChgInHollow {
    pub hp_lost: i32,
    pub hp_add: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct AvatarUnitInfo {
    pub uid: u64,
    pub properties_uid: u64,
    pub is_banned: bool,
    pub modified_property: PropertyDoubleKeyHashMap<u64, PropertyType, i32>,
    pub hp_lost_hollow: i32,
    pub hp_add_hollow: i32,
    pub layer_property_change: PropertyHashMap<i32, AvatarPropertyChgInHollow>,
}

#[derive(OctData, Clone, Debug)]
pub struct BuddyUnitInfo {
    pub uid: u64,
    pub properties: u64,
}

#[derive(OctData, Clone, Debug)]
pub struct DungeonDropPollInfo {
    pub action_card_mask: PropertyHashMap<i32, i32>,
}

#[derive(OctData, Clone, Debug)]
pub struct BattleReport {
    pub index: i32,
    pub report_type: ReportType,
    pub id: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct DungeonInfo {
    pub uid: u64,
    pub id: i32,
    pub default_scene_uid: u64,
    pub start_timestamp: u64,
    pub to_be_destroyed: bool,
    pub back_scene_uid: u64,
    pub quest_collection_uid: u64,
    pub avatars: PropertyHashMap<u64, AvatarUnitInfo>,
    pub buddy: BuddyUnitInfo,
    pub world_quest_id: i32,
    pub scene_properties_uid: u64,
    pub drop_poll_chg_infos: PropertyHashMap<DungeonContentDropPoolType, DungeonDropPollInfo>,
    pub is_in_dungeon: bool,
    pub initiative_item: i32,
    pub initiative_item_used_times: i32,
    pub avatar_map: PropertyHashMap<i8, AvatarUnitInfo>,
    pub battle_report: Vec<BattleReport>,
    pub dungeon_group_uid: u64,
    pub entered_times: u16,
    pub is_preset_avatar: bool,
    pub hollow_event_version: i32,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct DungeonCollection {
    #[tag = 1]
    pub dungeons: Option<PropertyHashMap<u64, DungeonInfo>>,
    #[tag = 2]
    pub scenes: Option<PropertyHashMap<u64, SceneInfo>>,
    #[tag = 3]
    pub default_scene_uid: Option<u64>,
    #[tag = 4]
    pub transform: Option<Transform>,
    #[tag = 5]
    pub used_story_mode: Option<bool>,
    #[tag = 6]
    pub used_manual_qte_mode: Option<bool>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct QuestData {
    #[tag = 1]
    pub quests: Option<PropertyDoubleKeyHashMap<u64, i32, QuestInfo>>,
    #[tag = 2]
    pub world_quest_for_cur_dungeon: Option<i32>,
    #[tag = 3]
    pub world_quest_collection_uid: Option<u64>,
    #[tag = 4]
    #[skip_property]
    pub unlock_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 5]
    pub is_afk: Option<bool>,
    #[tag = 6]
    pub world_quest_for_cur_dungeon_afk: Option<i32>,
}

#[derive(OctData, Clone, Debug)]
pub struct VideotapeInfo {
    pub star_count: PropertyHashMap<u8, u16>,
    pub finished: bool,
    pub awarded_star: PropertyHashMap<u8, HashSet<u16>>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct ArchiveInfo {
    #[tag = 1]
    pub videotapes_info: Option<PropertyHashMap<i32, VideotapeInfo>>,
}

#[derive(OctData, Clone, Debug)]
pub struct AutoRecoveryInfo {
    pub last_recovery_timestamp: u64,
    pub buy_times: u32,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct UnlockInfo {
    #[tag = 1]
    pub unlocked_list: Option<PropertyHashSet<i32>>,
    #[tag = 2]
    #[skip_property]
    pub condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct YorozuyaInfo {
    #[tag = 1]
    pub last_refresh_timestamp_common: Option<u64>,
    #[tag = 2]
    pub yorozuya_level: Option<u32>,
    #[tag = 3]
    pub yorozuya_rank: Option<u32>,
    #[tag = 4]
    pub gm_quests: Option<PropertyHashMap<HollowQuestType, Vec<i32>>>,
    #[tag = 5]
    pub gm_enabled: Option<bool>,
    #[tag = 6]
    pub hollow_quests: Option<PropertyDoubleKeyHashMap<i32, HollowQuestType, PropertyHashSet<i32>>>,
    #[tag = 7]
    pub urgent_quests_queue: Option<PropertyHashMap<i32, Vec<i32>>>,
    #[tag = 8]
    pub last_refresh_timestamp_urgent: Option<u64>,
    #[tag = 9]
    pub next_refresh_timestamp_urgent: Option<u64>,
    #[tag = 10]
    pub finished_hollow_quest_count: Option<i32>,
    #[tag = 11]
    pub finished_hollow_quest_count_of_type: Option<PropertyHashMap<i16, i32>>,
    #[tag = 12]
    pub unlock_hollow_id: Option<Vec<i32>>,
    #[tag = 13]
    #[skip_property]
    pub unlock_hollow_id_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct EquipGachaInfo {
    #[tag = 1]
    pub smithy_level: Option<i32>,
    #[tag = 2]
    pub security_num_by_lv: Option<PropertyHashMap<i32, i32>>,
    #[tag = 3]
    #[skip_property]
    pub total_gacha_times: Option<i32>,
    #[tag = 4]
    #[skip_property]
    pub equip_star_up_times: Option<i32>,
    #[tag = 5]
    #[skip_property]
    pub avatar_level_advance_times: Option<i32>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct BeginnerProcedureInfo {
    #[tag = 1]
    pub procedure_info: Option<i32>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct PlayerPosInMainCity {
    #[tag = 1]
    pub position: Option<Vector3f>,
    #[tag = 2]
    pub rotation: Option<Vector3f>,
    #[tag = 3]
    pub initial_pos_id: Option<i32>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct FairyInfo {
    #[tag = 1]
    pub fairy_groups: Option<PropertyHashMap<i32, FairyState>>,
    #[tag = 2]
    #[skip_property]
    pub condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct PopupWindowInfo {
    #[tag = 1]
    pub popup_window_list: Option<Vec<i32>>,
    #[tag = 2]
    #[skip_property]
    pub condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct TipsInfo {
    #[tag = 1]
    pub tips_list: Option<Vec<i32>>,
    #[tag = 2]
    #[skip_property]
    pub tips_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 3]
    pub tips_group: Option<Vec<i32>>,
    #[tag = 4]
    #[skip_property]
    pub tips_group_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct MainCityQuestData {
    #[tag = 1]
    pub exicing_finish_script_group: Option<Vec<i32>>,
    #[tag = 2]
    pub in_progress_quests: Option<Vec<i32>>,
}

#[derive(OctData, Clone, Debug)]
pub struct EmbattleInfo {
    pub avatars: Vec<i32>,
    pub buddy: i32,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct Embattles {
    #[tag = 1]
    pub last_embattles: Option<PropertyHashMap<QuestType, EmbattleInfo>>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct DayChangeInfo {
    #[tag = 1]
    pub last_daily_refresh_timing: Option<u64>,
}

#[derive(OctData, Clone, Debug)]
pub struct InteractInfo {
    pub interact_id: i32,
    pub interact_shape: u16,
    pub scale_x: f64,
    pub scale_y: f64,
    pub scale_z: f64,
    pub name: String,
    pub participators: PropertyHashMap<i32, String>,
    pub scale_w: f64,
    pub scale_r: f64,
}

#[derive(OctData, Clone, Debug)]
pub struct EventGraphsInfo {
    pub event_graphs_info: PropertyHashMap<i32, EventGraphInfo>,
    pub default_event_graph_id: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct PlayerNPCInfo {
    pub interact_info: InteractInfo,
    pub npc_uid: u64,
    pub event_graphs_info: EventGraphsInfo,
    pub npc_tag_id: i32,
    pub vhs_trending_id: i32,
    pub visible: bool,
    pub invisible_by_quest: PropertyHashSet<i32>,
    pub look_ik: bool,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct PlayerNPCsInfo {
    #[tag = 1]
    pub npcs_info: Option<PropertyHashMap<u64, PlayerNPCInfo>>,
    #[tag = 2]
    pub destroy_npc_when_leave_section: Option<PropertyHashSet<u64>>,
}

#[derive(OctData, Clone, Debug)]
pub struct ToExecuteScriptInfo {
    pub remove_after_finish: bool,
    pub specials: PropertyHashMap<String, i64>,
    pub event_graphs: PropertyHashSet<i32>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct MUIPData {
    #[tag = 1]
    pub ban_begin_time: Option<String>,
    #[tag = 2]
    pub ban_end_time: Option<String>,
    #[tag = 3]
    pub tag_value: Option<u64>,
    #[tag = 4]
    pub dungeon_enter_times: Option<PropertyHashMap<i32, i32>>,
    #[tag = 5]
    pub scene_enter_times: Option<PropertyHashMap<i32, i32>>,
    #[tag = 6]
    pub dungeon_pass_times: Option<PropertyHashMap<i32, i32>>,
    #[tag = 7]
    pub scene_pass_times: Option<PropertyHashMap<i32, i32>>,
    #[tag = 8]
    pub alread_cmd_uids: Option<PropertyHashSet<u64>>,
    #[tag = 9]
    pub game_total_time: Option<u64>,
    #[tag = 10]
    pub language_type: Option<u16>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct RamenData {
    #[tag = 1]
    pub unlock_ramen: Option<PropertyHashSet<i32>>,
    #[tag = 2]
    pub cur_ramen: Option<i32>,
    #[tag = 3]
    pub used_times: Option<i32>,
    #[tag = 4]
    pub unlock_initiative_item: Option<PropertyHashSet<i32>>,
    #[tag = 5]
    #[skip_property]
    pub unlock_ramen_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 6]
    #[skip_property]
    pub unlock_item_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 7]
    pub has_mystical_spice: Option<bool>,
    #[tag = 8]
    #[skip_property]
    pub unlock_has_mystical_spice_condition_progress: Option<PropertyHashMap<i32, i32>>,
    #[tag = 9]
    pub cur_mystical_spice: Option<i32>,
    #[tag = 10]
    pub unlock_mystical_spice: Option<PropertyHashSet<i32>>,
    #[tag = 11]
    #[skip_property]
    pub unlock_mystical_spice_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 12]
    pub unlock_initiative_item_group: Option<PropertyHashSet<i32>>,
    #[tag = 13]
    pub hollow_item_history: Option<PropertyHashMap<i32, i32>>,
    #[tag = 14]
    pub initial_item_ability: Option<u64>,
    #[tag = 15]
    #[property_object(u8, 0x01)]
    pub new_unlock_ramen: Option<Vec<i32>>,
    #[tag = 16]
    #[skip_property]
    pub eat_ramen_times: Option<i32>,
    #[tag = 17]
    #[skip_property]
    pub make_hollow_item_times: Option<i32>,
    #[tag = 18]
    pub new_unlock_initiative_item: Option<PropertyHashSet<i32>>,
}

#[derive(OctData, Clone, Debug)]
pub struct GoodsInfo {
    pub id: i32,
    pub purchased_num: u32,
    pub last_refresh_time: u64,
    pub discount: u16,
}

#[derive(OctData, Clone, Debug)]
pub struct ShelfInfo {
    pub id: i32,
    pub custom_goods_in_shelf: PropertyHashSet<i32>,
    pub goods_info: PropertyHashMap<i32, GoodsInfo>,
}

#[derive(OctData, Clone, Debug)]
pub struct ShopInfo {
    pub id: i32,
    pub shelf_info: PropertyHashMap<i32, ShelfInfo>,
    pub refreshed_count: i32,
    pub last_refresh_time: u64,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct ShopsInfo {
    #[tag = 1]
    pub vip_level: Option<u8>,
    #[tag = 2]
    #[skip_property]
    pub shops: Option<PropertyHashMap<i32, ShopInfo>>,
    #[tag = 3]
    #[skip_property]
    pub shop_buy_times: Option<i32>,
}

#[derive(OctData, Clone, Debug)]
pub struct VHSTrendingInfo {
    pub trend_id: i32,
    pub state: u16,
    pub match_level: u16,
    pub is_accept: bool,
}

#[derive(OctData, Clone, Debug)]
pub struct VHSTrendingCfgInfo {
    pub trend_id: i32,
    pub complete_level: i16,
    pub know_state: i16,
}

#[derive(OctData, Clone, Debug)]
pub struct VHSNpcInfo {
    pub npc_id: i32,
    pub state: i16,
    pub new_know: bool,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct VHSStoreData {
    #[tag = 1]
    pub store_level: Option<i32>,
    #[tag = 2]
    pub unreceived_reward: Option<i32>,
    #[tag = 3]
    #[skip_property]
    pub hollow_enter_times: Option<i32>,
    #[tag = 4]
    pub last_receive_time: Option<i32>,
    #[tag = 5]
    #[property_object(u8, 0x01)]
    pub vhs_collection_slot: Option<Vec<i32>>,
    #[tag = 6]
    pub unlock_vhs_collection: Option<PropertyHashSet<i32>>,
    #[tag = 7]
    #[skip_property]
    pub already_trending: Option<PropertyHashSet<i32>>,
    #[tag = 8]
    #[skip_property]
    pub unlock_trending_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 9]
    pub is_need_refresh: Option<bool>,
    #[tag = 10]
    #[skip_property]
    pub scripts_id: Option<PropertyHashSet<i32>>,
    #[tag = 11]
    pub store_exp: Option<i32>,
    #[tag = 12]
    pub is_level_chg_tips: Option<bool>,
    #[tag = 13]
    #[skip_property]
    pub vhs_hollow: Option<Vec<i32>>,
    #[tag = 14]
    #[skip_property]
    pub is_receive_trending_reward: Option<bool>,
    #[tag = 15]
    #[skip_property]
    pub is_need_first_trending: Option<bool>,
    #[tag = 16]
    #[skip_property]
    pub last_basic_script: Option<i32>,
    #[tag = 17]
    #[skip_property]
    pub is_complete_first_trending: Option<bool>,
    #[tag = 18]
    #[skip_property]
    pub last_basic_npc: Option<u64>,
    #[tag = 19]
    #[skip_property]
    pub can_random_trending: Option<PropertyHashSet<i32>>,
    #[tag = 20]
    #[property_object(u8, 0x01)]
    pub vhs_trending_info: Option<Vec<VHSTrendingInfo>>,
    #[tag = 21]
    pub unlock_vhs_trending_info: Option<PropertyHashMap<i32, VHSTrendingCfgInfo>>,
    #[tag = 22]
    pub vhs_flow: Option<i32>,
    #[tag = 23]
    pub received_reward: Option<i32>,
    #[tag = 24]
    pub last_reward: Option<i32>,
    #[tag = 25]
    pub last_exp: Option<i32>,
    #[tag = 26]
    pub last_flow: Option<i32>,
    #[tag = 27]
    #[property_object(u8, 0x01)]
    pub last_vhs_trending_info: Option<Vec<VHSTrendingInfo>>,
    #[tag = 28]
    #[property_object(u8, 0x01)]
    pub new_know_trend: Option<Vec<i32>>,
    #[tag = 29]
    #[skip_property]
    pub quest_finish_script: Option<PropertyDoubleKeyHashMap<i32, i32, HashMap<String, u64>>>,
    #[tag = 30]
    #[skip_property]
    pub quest_finish_scripts_id: Option<PropertyHashSet<i32>>,
    #[tag = 31]
    #[skip_property]
    pub total_received_reward: Option<PropertyHashMap<i32, i32>>,
    #[tag = 32]
    #[property_object(u8, 0x01)]
    pub last_vhs_npc_info: Option<Vec<VHSNpcInfo>>,
    #[tag = 33]
    #[skip_property]
    pub vhs_npc_info: Option<Vec<VHSNpcInfo>>,
    #[tag = 34]
    #[skip_property]
    pub npc_info: Option<PropertyHashSet<i32>>,
    #[tag = 35]
    #[skip_property]
    pub total_received_reward_times: Option<i32>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct OperationMailReceiveInfo {
    #[tag = 1]
    pub receive_list: Option<PropertyHashSet<i32>>,
    #[tag = 2]
    pub condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct PayInfo {
    #[tag = 1]
    pub month_total_pay: Option<i32>,
}

#[derive(OctData, Clone, Debug)]
pub struct NpcSceneData {
    pub section_id: i32,
    pub transform: Transform,
}

#[derive(OctData, Clone, Debug)]
pub struct NpcInfo {
    pub uid: u64,
    pub id: i32,
    pub tag_value: i32,
    pub scene_uid: u64,
    pub parent_uid: u64,
    pub owner_uid: u64,
    pub scene_data: NpcSceneData,
    pub references: PropertyHashSet<u64>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct BattleEventInfo {
    #[tag = 1]
    #[skip_property]
    pub unlock_battle: Option<PropertyHashSet<i32>>,
    #[tag = 2]
    #[skip_property]
    pub unlock_battle_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 3]
    #[skip_property]
    pub alread_rand_battle: Option<PropertyDoubleKeyHashMap<i32, i32, HashSet<i32>>>,
    #[tag = 4]
    pub rand_battle_type: Option<PropertyHashMap<i32, HollowBattleEventType>>,
    #[tag = 5]
    #[property_object(u8, 0x01)]
    pub alread_battle_stage: Option<Vec<String>>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct GMData {
    #[tag = 1]
    #[skip_property]
    pub condition_proress: Option<PropertyDoubleKeyHashMap<String, i32, i32>>,
    #[tag = 2]
    #[skip_property]
    pub completed_conditions: Option<PropertyHashSet<String>>,
    #[tag = 3]
    #[skip_property]
    pub register_conditions: Option<PropertyHashSet<String>>,
}

#[derive(OctData, Clone, Debug)]
pub struct PlayerMailExtInfo {
    pub timestamp: u64,
    pub mail_state: MailState,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct PlayerMailExtInfos {
    #[tag = 1]
    pub player_mail_ext_info: Option<PropertyHashMap<String, PlayerMailExtInfo>>,
}

#[derive(OctData, Clone, Debug)]
pub struct DungeonTable {
    pub uid: u64,
    pub id: i32,
    pub begin_timestamp: u64,
    pub dungeon_ext: DungeonTableExt,
    pub to_be_destroyed: bool,
}

#[derive(OctData, Clone, Debug)]
pub struct SceneTable {
    pub uid: u64,
    pub id: i32,
    pub begin_timestamp: u64,
    pub scene_ext: SceneTableExt,
    pub to_be_destroyed: bool,
}

#[derive(OctData, Clone, Debug)]
pub struct SectionInfo {
    pub id: i32,
    pub scene_uid: u64,
    pub event_graphs_info: EventGraphsInfo,
    pub section_info_ext: SectionInfoExt,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct SingleDungeonGroup {
    #[tag = 1]
    pub dungeons: Option<PropertyHashMap<u64, DungeonTable>>,
    #[tag = 2]
    pub scenes: Option<PropertyDoubleKeyHashMap<u64, u64, SceneTable>>,
    #[tag = 3]
    pub section: Option<PropertyDoubleKeyHashMap<u64, i32, SectionInfo>>,
    #[tag = 4]
    pub npcs: Option<PropertyDoubleKeyHashMap<u64, u64, NpcInfo>>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct NewbieInfo {
    #[tag = 1]
    pub unlocked_id: Option<PropertyHashSet<i32>>,
    #[tag = 2]
    #[skip_property]
    pub condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct LoadingPageTipsInfo {
    #[tag = 1]
    pub unlocked_id: Option<PropertyHashSet<i32>>,
    #[tag = 2]
    #[skip_property]
    pub condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct CollectMap {
    #[tag = 1]
    pub card_map: Option<PropertyHashSet<i32>>,
    #[tag = 2]
    pub curse_map: Option<PropertyHashSet<i32>>,
    #[tag = 3]
    pub event_icon_map: Option<PropertyHashSet<i32>>,
    #[tag = 4]
    #[skip_property]
    pub unlock_cards: Option<PropertyHashSet<i32>>,
    #[tag = 5]
    #[skip_property]
    pub unlock_card_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 6]
    #[skip_property]
    pub unlock_curses: Option<PropertyHashSet<i32>>,
    #[tag = 7]
    #[skip_property]
    pub unlock_curse_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 8]
    #[skip_property]
    pub unlock_events: Option<PropertyHashSet<i32>>,
    #[tag = 9]
    #[skip_property]
    pub unlock_event_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 10]
    #[skip_property]
    pub unlock_event_icons: Option<PropertyHashSet<i32>>,
    #[tag = 11]
    #[skip_property]
    pub unlock_event_icon_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 12]
    pub new_card_map: Option<PropertyHashSet<i32>>,
    #[tag = 13]
    pub new_curse_map: Option<PropertyHashSet<i32>>,
    #[tag = 14]
    pub new_event_icon_map: Option<PropertyHashSet<i32>>,
}

#[derive(OctData, Clone, Debug)]
pub struct AreaNPCInfo {
    pub tag_id: i32,
    pub interacts: PropertyHashSet<i32>,
}

#[derive(OctData, Clone, Debug)]
pub struct AreaOwnerInfo {
    pub owner_type: u16,
    pub owner_id: i32,
    pub npcs: PropertyHashMap<u64, AreaNPCInfo>,
    pub sequence: u32,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct AreasInfo {
    #[tag = 1]
    pub area_owners_info: Option<PropertyDoubleKeyHashMap<u16, i32, AreaOwnerInfo>>,
    #[tag = 2]
    pub sequence: Option<u32>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct BGMInfo {
    #[tag = 1]
    pub bgm_id: Option<i32>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct HollowInfo {
    #[tag = 1]
    pub banned_hollow_event: Option<PropertyHashSet<i32>>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object(u16, 0x01)]
#[root]
pub struct PlayerInfo {
    #[tag = 1]
    pub uid: Option<u64>,
    #[tag = 2]
    pub account_name: Option<String>,
    #[tag = 3]
    pub last_enter_world_timestamp: Option<u64>,
    #[tag = 4]
    pub items: Option<PropertyHashMap<u64, ItemInfo>>,
    #[tag = 5]
    pub dungeon_collection: Option<DungeonCollection>,
    #[tag = 6]
    #[skip_property]
    pub properties: Option<PropertyDoubleKeyHashMap<u64, u16, i32>>,
    #[tag = 7]
    pub scene_properties: Option<PropertyDoubleKeyHashMap<u64, u16, i32>>,
    #[tag = 8]
    pub quest_data: Option<QuestData>,
    #[tag = 9]
    pub joined_chat_rooms: Option<Vec<u64>>,
    #[tag = 10]
    pub scene_uid: Option<u64>,
    #[tag = 11]
    pub archive_info: Option<ArchiveInfo>,
    #[tag = 12]
    pub auto_recovery_info: Option<PropertyHashMap<i32, AutoRecoveryInfo>>,
    #[tag = 13]
    pub unlock_info: Option<UnlockInfo>,
    #[tag = 14]
    pub yorozuya_info: Option<YorozuyaInfo>,
    #[tag = 15]
    pub equip_gacha_info: Option<EquipGachaInfo>,
    #[tag = 16]
    pub beginner_procedure_info: Option<BeginnerProcedureInfo>,
    #[tag = 17]
    pub pos_in_main_city: Option<PlayerPosInMainCity>,
    #[tag = 18]
    pub fairy_info: Option<FairyInfo>,
    #[tag = 19]
    pub popup_window_info: Option<PopupWindowInfo>,
    #[tag = 20]
    pub tips_info: Option<TipsInfo>,
    #[tag = 21]
    pub main_city_quest_data: Option<MainCityQuestData>,
    #[tag = 22]
    pub embattles: Option<Embattles>,
    #[tag = 23]
    #[skip_property]
    pub day_change_info: Option<DayChangeInfo>,
    #[tag = 24]
    #[skip_property]
    pub npcs_info: Option<PlayerNPCsInfo>,
    #[tag = 25]
    #[skip_property]
    pub scripts_to_execute: Option<PropertyDoubleKeyHashMap<i32, i32, ToExecuteScriptInfo>>,
    #[tag = 26]
    #[skip_property]
    pub scripts_to_remove: Option<PropertyHashMap<i32, PropertyHashSet<i32>>>,
    #[tag = 27]
    pub last_leave_world_timestamp: Option<u64>,
    #[tag = 28]
    #[skip_property]
    pub muip_data: Option<MUIPData>,
    #[tag = 29]
    pub nick_name: Option<String>,
    #[tag = 30]
    pub ramen_data: Option<RamenData>,
    #[tag = 31]
    pub shop: Option<ShopsInfo>,
    #[tag = 32]
    pub vhs_store_data: Option<VHSStoreData>,
    #[tag = 33]
    #[skip_property]
    pub operation_mail_receive_info: Option<OperationMailReceiveInfo>,
    #[tag = 34]
    pub second_last_enter_world_timestamp: Option<u64>,
    #[tag = 35]
    pub login_times: Option<i32>,
    #[tag = 36]
    pub create_timestamp: Option<u64>,
    #[tag = 37]
    pub gender: Option<u8>,
    #[tag = 38]
    pub avatar_id: Option<i32>,
    #[tag = 39]
    pub prev_scene_uid: Option<u64>,
    #[tag = 40]
    pub register_cps: Option<String>,
    #[tag = 41]
    pub register_platform: Option<i32>,
    #[tag = 42]
    pub pay_info: Option<PayInfo>,
    #[tag = 43]
    #[skip_property]
    pub private_npcs: Option<PropertyHashMap<u64, NpcInfo>>,
    #[tag = 44]
    pub battle_event_info: Option<BattleEventInfo>,
    #[tag = 45]
    pub gm_data: Option<GMData>,
    #[tag = 46]
    #[skip_property]
    pub player_mail_ext_infos: Option<PlayerMailExtInfos>,
    #[tag = 47]
    #[skip_property]
    pub single_dungeon_group: Option<SingleDungeonGroup>,
    #[tag = 48]
    pub newbie_info: Option<NewbieInfo>,
    #[tag = 49]
    pub loading_page_tips_info: Option<LoadingPageTipsInfo>,
    #[tag = 50]
    pub switch_of_story_mode: Option<bool>,
    #[tag = 51]
    pub switch_of_qte: Option<bool>,
    #[tag = 52]
    pub collect_map: Option<CollectMap>,
    #[tag = 53]
    pub areas_info: Option<AreasInfo>,
    #[tag = 54]
    pub bgm_info: Option<BGMInfo>,
    #[tag = 55]
    pub main_city_objects_state: Option<PropertyHashMap<i32, i32>>,
    #[tag = 56]
    pub hollow_info: Option<HollowInfo>,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
pub struct PropertyKeyValue {
    pub key: PropertyType,
    pub value: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct HollowShopModification {
    pub ability_modified_num: PropertyDoubleKeyHashMap<HollowShopType, String, i32>,
    pub action_modified_num: PropertyHashMap<HollowShopType, i32>,
    pub overwrite_price: PropertyHashMap<HollowShopType, i32>,
}

#[derive(OctData, Clone, Debug)]
pub struct HollowInitialStateOfPlayer {
    pub rogue_like_items: Vec<ItemInfo>,
    pub properties: PropertyDoubleKeyHashMap<u64, u16, i32>,
}

#[derive(OctData, Clone, Debug)]
pub struct PlayerHollowSectionInfo {
    pub prev_grid_index: u16,
    pub cur_grid_index: u16,
    pub entered_times: u8,
    //pub initial_state_of_player: HollowInitialStateOfPlayer,
    pub global_event: u64,
    pub perform_event_graph: u64,
    pub pos_before_move: u16,
}

#[derive(OctData, Clone, Debug)]
pub struct EventStackFrame {
    pub action_info: ActionInfo,
    pub action_id: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct EventInfo {
    pub id: i32,
    pub cur_action_id: i32,
    pub action_move_path: Vec<i32>,
    pub state: EventState,
    pub prev_state: EventState,
    pub cur_action_info: ActionInfo,
    pub cur_action_state: ActionState,
    pub predicated_failed_actions: PropertyHashSet<i32>,
    pub stack_frames: Vec<EventStackFrame>, // CPLinkedList?
}

#[derive(Clone, Debug)]
pub struct HollowEventGraphInfo {
    // abstract EventGraphInfo part
    pub config_id: i32,
    pub events_info: PropertyHashMap<i32, EventInfo>,
    pub specials: PropertyHashMap<String, u64>,
    pub is_new: bool,
    pub finished: bool,
    pub list_specials: PropertyHashMap<String, Vec<u64>>,

    // HollowEventGraphInfo part
    pub fired_count: u8,
    pub hollow_event_template_id: i32,
    pub uid: u64,
    pub is_create_by_gm: bool,
}

impl std::fmt::Display for HollowEventGraphInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "HollowEventGraphInfo.")
    }
}

// AUTISM!!!
impl OctData for HollowEventGraphInfo {
    fn marshal_to<W: std::io::Write>(
        &self,
        w: &mut W,
        bt_property_tag: u16,
    ) -> std::io::Result<()> {
        if bt_property_tag == 0 {
            6_u16.marshal_to(w, bt_property_tag)?;
        }

        self.config_id.marshal_to(w, bt_property_tag)?;
        self.events_info.marshal_to(w, bt_property_tag)?;
        self.specials.marshal_to(w, bt_property_tag)?;
        self.is_new.marshal_to(w, bt_property_tag)?;
        self.finished.marshal_to(w, bt_property_tag)?;
        self.list_specials.marshal_to(w, bt_property_tag)?;

        if bt_property_tag == 0 {
            4_u16.marshal_to(w, bt_property_tag)?;
        }

        self.fired_count.marshal_to(w, bt_property_tag)?;
        self.hollow_event_template_id
            .marshal_to(w, bt_property_tag)?;
        self.uid.marshal_to(w, bt_property_tag)?;
        self.is_create_by_gm.marshal_to(w, bt_property_tag)?;

        Ok(())
    }

    fn unmarshal_from<R: std::io::Read>(r: &mut R, bt_property_tag: u16) -> std::io::Result<Self> {
        Ok(Self {
            config_id: {
                if bt_property_tag == 0 {
                    assert!(u16::unmarshal_from(r, bt_property_tag)? == 6);
                }

                OctData::unmarshal_from(r, bt_property_tag)?
            },
            events_info: OctData::unmarshal_from(r, bt_property_tag)?,
            specials: OctData::unmarshal_from(r, bt_property_tag)?,
            is_new: OctData::unmarshal_from(r, bt_property_tag)?,
            finished: OctData::unmarshal_from(r, bt_property_tag)?,
            list_specials: OctData::unmarshal_from(r, bt_property_tag)?,

            fired_count: {
                if bt_property_tag == 0 {
                    assert!(u16::unmarshal_from(r, bt_property_tag)? == 4);
                }

                OctData::unmarshal_from(r, bt_property_tag)?
            },
            hollow_event_template_id: OctData::unmarshal_from(r, bt_property_tag)?,
            uid: OctData::unmarshal_from(r, bt_property_tag)?,
            is_create_by_gm: OctData::unmarshal_from(r, bt_property_tag)?,
        })
    }
}

#[derive(OctData, Clone, Debug)]
pub struct PrepareSection {
    pub section_id: i32,
    pub initial_pos: u16,
    pub show_other: bool,
    pub battle_end_goto_next_hollow: bool,
}

#[derive(OctData, Clone, Debug)]
pub struct AbilityModifierInfo {
    pub uid: u64,
    pub added_scene_property: PropertyHashMap<ScenePropertyType, i32>,
}

#[derive(OctData, Clone, Debug)]
pub struct AbilityInfo {
    pub id: String,
    pub specials: PropertyHashMap<String, i64>,
    pub modifiers_info: PropertyHashMap<String, AbilityModifierInfo>,
    pub stack_num: i32,
    pub disabled: bool,
    pub sequence: u16,
}

#[derive(OctData, Clone, Debug)]
pub struct AbilitiesInfo {
    pub abilities: PropertyHashMap<u64, AbilityInfo>,
    pub sequence_no: u16,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq, Hash)]
pub struct HollowDungeonAvatarInfo {
    pub uid: u64,
    pub properties_uid: u64,
}

#[derive(OctData, Clone, Debug)]
pub struct HollowDungeonBuddyInfo {
    pub uid: u64,
    pub properties_uid: u64,
}

#[derive(OctData, Clone, Debug)]
pub struct HollowLevelInfo {
    pub id: i32,
    pub chessboard_id: i32,
    pub dependent_levels: Vec<u8>,
    pub layer: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct ToDoEventInfo {
    pub event_graph_uid: u64,
    pub start_node: String,
    pub event_id: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct HollowGridMapInfo {
    pub grids: PropertyHashMap<u16, HollowGridInfo>,
    pub row_num: u8,
    pub col_num: u8,
    pub main_path: Vec<u16>,
    pub alt_paths: Vec<Vec<u16>>,
    pub ring: PropertyHashSet<u16>,
    pub shop_info: PropertyDoubleKeyHashMap<u16, HollowShopType, ConfigShopInfo>,
    pub to_do_event_list: Vec<ToDoEventInfo>,
    pub start_grid: u16,
    pub end_grid: u16,
}

#[derive(OctData, Clone, Debug)]
pub struct ChoiceInfo {
    pub id: i32,
    pub hide_info: bool,
    pub forbidden: bool,
}

#[derive(OctData, Clone, Debug)]
pub struct HollowGridMapProtocolInfo {
    pub row: u8,
    pub col: u8,
    pub start_grid: u16,
    pub grids: PropertyHashMap<u16, HollowGridProtocolInfo>,
    pub chessboard_id: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct HollowGridProtocolInfo {
    pub grid: HollowGridInfo,
    pub event_type: HollowEventType,
    pub use_perform: bool,
}

#[derive(OctData, Clone, Debug)]
pub struct HollowGridInfo {
    pub flag: i32,   // HollowGridFlag
    pub link_to: i8, // HollowGridLink
    pub event_graph_info: HollowEventGraphInfo,
    pub travelled_count: u16,
    pub node_state: NodeState,
    pub node_visible: NodeVisible,
}

#[derive(OctData, Clone, Debug)]
pub struct ConfigShopInfo {
    pub goods: Vec<ConfigItem>,
    pub currency: HollowShopCurrency,
}

#[derive(OctData, Clone, Debug)]
pub struct ConfigItem {
    pub uid: i32,
    pub item_id: i32,
    pub count: i32,
    pub value: i32,
    pub base_value: i32,
    pub discount: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct EventListenerInfo {
    pub event_graph_id: i32,
    pub events_to_trigger: Vec<String>,
}

#[derive(OctData, Clone, Debug)]
pub struct BoundNPCAndInteractInfo {
    pub is_bound_npc: bool,
    pub interacts: PropertyHashSet<i32>,
    pub npc_reference_uid: u64,
}

#[derive(OctData, Clone, Debug)]
pub struct LogSkillUseInfo {
    pub skill_name: String,
    pub damage: i32,
    pub level: u8,
    pub use_times: i32,
    pub hit_times: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct LogBattleAvatarInfo {
    pub avatar_id: i32,
    pub avatar_uid: i64,
    pub power: i32,
    pub is_live: u8,
    pub max_hp: i32,
    pub hp: i32,
    pub damage: i32,
    pub be_damage: i32,
    pub be_hit: i32,
    pub dodge: i32,
    pub succ_dodge: i32,
    pub resident: i32,
    pub dizzier: i32,
    pub start_hp: i32,
    pub skill_use: Vec<LogSkillUseInfo>,
}

#[derive(OctData, Clone, Debug)]
pub struct LogMonsterSkillUseInfo {
    pub skill_name: String,
    pub damage: i32,
    pub use_times: i32,
    pub hit_times: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct LogMonsterInfo {
    pub monster_id: i32,
    pub monster_uid: i64,
    pub damage: i32,
    pub live_time: i32,
    pub be_dizzier_times: i32,
    pub skill_use: Vec<LogMonsterSkillUseInfo>,
}

#[derive(OctData, Clone, Debug)]
pub struct LogTrapInfo {
    pub trap_id: i32,
    pub trap_uid: i64,
    pub damage: i32,
    pub live_time: i32,
    pub is_trigger: u8,
}

#[derive(OctData, Clone, Debug)]
pub struct LogBrokeItemInfo {
    pub broke_id: i32,
    pub broke_uid: i64,
    pub damage: i32,
    pub live_time: i32,
    pub is_broke: u8,
}

#[derive(OctData, Clone, Debug)]
pub struct LogBattleStatistics {
    pub battle_uid: i64,
    pub battle_id: i32,
    pub pass_time: i32,
    pub result: u8,
    pub switch_num: i32,
    pub score: u8,
    pub avatar_list: Vec<LogBattleAvatarInfo>,
    pub monster_list: Vec<LogMonsterInfo>,
    pub trap_list: Vec<LogTrapInfo>,
    pub broke_item_list: Vec<LogBrokeItemInfo>,
    pub star: u8,
}
