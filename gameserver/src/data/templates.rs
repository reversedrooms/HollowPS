#![allow(dead_code)]
use super::tsv_util::from_sequence;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MainCityObjectTemplate {
    #[serde(rename = "TagID")]
    pub tag_id: i32,
    #[serde(rename = "NPCID")]
    pub npc_id: i32,
    pub create_position: String,
    pub create_type: i32,
    #[serde(rename = "DefaultInteractIDs")]
    #[serde(deserialize_with = "from_sequence")]
    pub default_interact_ids: Vec<i32>,
    pub interact_name: Option<String>,
    pub interact_shape: i32,
    pub interact_scale: String,
    pub fan_interact_param: Option<String>,
    pub focus_interact_scale: f64,
    pub ignore_collider: bool,
    #[serde(rename = "LookIK")]
    pub look_ik: bool,
    #[serde(rename = "NPCLookIK")]
    pub npc_look_ik: bool,
    #[serde(rename = "SceneSoundID")]
    pub scene_sound_id: i32,
    pub player_rotate: bool,
    #[serde(rename = "NPCRotate")]
    pub npc_rotate: bool,
    pub scene_object_name: Option<String>,
    pub camera_story_key: Option<String>,
    pub action_state: i32,
    pub collider_state: Option<String>,
    pub item_state: Option<String>,
    #[serde(rename = "ObjectIDs")]
    #[serde(deserialize_with = "from_sequence")]
    pub object_ids: Vec<i32>,
    pub create_interval: i32,
    pub create_delay: i32,
    #[serde(rename = "NPCIcon")]
    pub npc_icon: Option<String>,
    pub action_switch: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnlockConfigTemplate {
    #[serde(rename = "ID")]
    pub id: i32,
    pub lock_type: i32,
    pub lock_param: Option<String>,
    pub unlock_type: i32,
    pub unlock_param: Option<String>,
    pub menu_type: i32,
    pub icon_res: Option<String>,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AvatarConfigTemplate {
    #[serde(rename = "ID")]
    pub id: i32,
    pub code_name: String,
    pub name: String,
    pub audio_event_replace_param: String,
    pub audio_bank: String,
    pub hud_icon_path: String,
    pub hollow_icon_path: String,
    pub icon_path: String,
    pub rect_icon_path: String,
    pub avatar_path: String,
    pub script_config_path: String,
    #[serde(rename = "UIScriptConfigPath")]
    pub uiscript_config_path: String,
    pub prefab_path: String,
    #[serde(rename = "UIPrefabPath")]
    pub uiprefab_path: String,
    pub main_page_show: bool,
    pub need_show: bool,
    #[serde(deserialize_with = "from_sequence")]
    pub hit_types: Vec<i32>,
    #[serde(deserialize_with = "from_sequence")]
    pub element_types: Vec<i32>,
    #[serde(deserialize_with = "from_sequence")]
    pub tags: Vec<String>,
    pub gender: i32,
    pub camp: i32,
    pub camp_name: Option<String>,
    pub group_icon_path: String,
    pub weapon_type: i32,
    pub star_initial: i32,
    #[serde(rename = "AvatarPieceID")]
    pub avatar_piece_id: i32,
    pub avatar_decompose: i32,
    pub avatar_compose: i32,
    #[serde(rename = "HP")]
    pub hp: i32,
    #[serde(rename = "HPGrowth")]
    pub hp_growth: i32,
    pub armor: i32,
    pub armor_growth: i32,
    pub shield: i32,
    pub shield_growth: i32,
    pub endurance: i32,
    pub attack: i32,
    pub attack_growth: i32,
    pub defence: i32,
    pub defence_growth: i32,
    pub crit: i32,
    pub crit_damage: i32,
    pub crit_res: i32,
    pub crit_dmg_res: i32,
    pub pen_rate: i32,
    pub pen_delta: i32,
    pub luck: i32,
    pub stun: i32,
    pub break_stun: i32,
    #[serde(rename = "SPBarPoint")]
    pub spbar_point: i32,
    pub sp_recover: i32,
    #[serde(rename = "RBL")]
    pub rbl: i32,
    #[serde(rename = "RBLCorrectionFactor")]
    pub rblcorrection_factor: i32,
    #[serde(rename = "RBLProbability")]
    pub rblprobability: i32,
    pub buff_resist_burn_possibility_ratio: i32,
    pub buff_resist_burn_possibility_delta: i32,
    pub buff_resist_frozen_possibility_ratio: i32,
    pub buff_resist_frozen_possibility_delta: i32,
    pub buff_resist_electric_possibility_ratio: i32,
    pub buff_resist_electric_possibility_delta: i32,
    pub weapon: i32,
    #[serde(rename = "RBP")]
    pub rbp: i32,
    #[serde(rename = "RBPFadeOutRate")]
    pub rbp_fade_out_rate: i32,
    #[serde(rename = "RBPFadeOutTime")]
    pub rbp_fade_out_time: i32,
    #[serde(rename = "RBPCorrectionFactor")]
    pub rbp_correction_factor: i32,
    #[serde(rename = "RBPProbability")]
    pub rbp_probability: i32,
}
