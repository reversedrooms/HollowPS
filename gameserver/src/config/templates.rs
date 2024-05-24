#![allow(dead_code)]
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
    pub object_ids: Vec<i32>,
    pub create_interval: i32,
    pub create_delay: i32,
    #[serde(rename = "NPCIcon")]
    pub npc_icon: Option<String>,
    pub action_switch: Option<String>,
}
