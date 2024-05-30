#![allow(unused)]

use std::collections::{BTreeMap, HashSet};

use protocol::{HollowEventType, NodeState, NodeVisible};
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigEventGraph {
    #[serde(rename = "ID")]
    pub id: i32,
    pub events: BTreeMap<ConfigEventType, ConfigEvent>,
}

#[derive(Deserialize, Default)]
#[serde(untagged)]
pub enum ConfigValue {
    Constant(i32),
    Expression(String),
    #[default]
    Empty,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ConfigEventType {
    OnStart,
    OnEnd,
    OnBro,
    OnSis,
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigEvent {
    #[serde(default)]
    pub actions: Vec<ConfigAction>,
}

#[derive(Deserialize, Default)]
pub enum HollowPositionOffsetType {
    #[default]
    Relative = 0,
    Absolute = 1,
    EventPos = 2,
}

#[derive(Deserialize, Default)]
#[serde(tag = "$type")]
pub enum ConfigAction {
    #[serde(rename = "Share.CConfigEmpty")]
    ConfigEmpty,
    #[serde(rename = "Share.CConfigSetMapState")]
    #[serde(rename_all = "PascalCase")]
    ConfigSetMapState {
        #[serde(default)]
        x: ConfigValue,
        #[serde(default)]
        y: ConfigValue,
        #[serde(default)]
        position: HollowPositionOffsetType,
        radius: Option<ConfigValue>,
        count: Option<ConfigValue>,
        #[serde(default)]
        r#type: HashSet<HollowEventType>,
        event_type_tag: Option<String>,
        #[serde(default)]
        from_visible_state: HashSet<NodeVisible>,
        #[serde(default)]
        to_visible_state: Vec<NodeVisible>,
        #[serde(default)]
        from_state: HashSet<NodeState>,
        #[serde(default)]
        to_state: Vec<NodeState>,
        #[serde(default)]
        exclude_player_pos: bool,
        #[serde(default)]
        index_list: Vec<ConfigValue>,
        #[serde(default)]
        use_perform: bool,
    },
    #[serde(rename = "Share.CConfigTriggerBattle")]
    #[serde(rename_all = "PascalCase")]
    ConfigTriggerBattle {
        #[serde(rename = "BattleID")]
        battle_id: ConfigValue,
        on_success: Option<String>,
        on_failure: Option<String>,
        #[serde(default)]
        end_hollow: bool,
        #[serde(default)]
        goto_next_hollow: bool,
    },
    #[serde(rename = "Share.CConfigFinishHollow")]
    ConfigFinishHollow,
    #[default]
    #[serde(other)]
    ConfigUnknown,
}
