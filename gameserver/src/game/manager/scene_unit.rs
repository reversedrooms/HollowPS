use std::{collections::HashMap, sync::Arc};

use anyhow::{bail, Result};
use protocol::*;
use qwer::{phashmap, PropertyHashMap};
use tokio::sync::RwLock;

use crate::data;

use super::UniqueIDManager;

pub struct SceneUnitManager {
    uid_mgr: Arc<UniqueIDManager>,
    units: RwLock<HashMap<u64, SceneUnitProtocolInfo>>,
}

impl SceneUnitManager {
    pub fn new(uid_mgr: Arc<UniqueIDManager>) -> Self {
        Self {
            uid_mgr,
            units: RwLock::new(HashMap::new()),
        }
    }

    pub async fn create_npc(
        &self,
        id: i32,
        tag: i32,
        quest_id: i32,
        interacts_info: PropertyHashMap<i32, InteractInfo>,
    ) -> u64 {
        let uid = self.uid_mgr.next();

        self.units.write().await.insert(
            uid,
            SceneUnitProtocolInfo::NpcProtocolInfo {
                uid,
                tag,
                id,
                quest_id,
                interacts_info,
            },
        );

        uid
    }

    pub async fn get(&self, uid: u64) -> Option<SceneUnitProtocolInfo> {
        self.units.read().await.get(&uid).map(|u| u.clone())
    }

    pub async fn sync(&self, scene_uid: u64, section_id: i32) -> PtcSyncSceneUnitArg {
        PtcSyncSceneUnitArg {
            scene_uid,
            section_id,
            is_partial: false,
            removed_scene_units: Vec::new(),
            scene_units: self
                .units
                .read()
                .await
                .iter()
                .map(|(_, unit)| unit.clone())
                .collect(),
        }
    }

    pub async fn add_scene_units(&self, section_id: i32) {
        for o in data::iter_main_city_object_collection().filter(|o| {
            o.create_type == 0
                && data::is_transform_in_section(&o.create_position, section_id)
                && o.default_interact_ids.len() != 0
                && !o.create_position.ends_with("_Test")
        }) {
            self.create_npc(
                o.npc_id,
                o.tag_id,
                0,
                PropertyHashMap::Base(
                    o.default_interact_ids
                        .iter()
                        .map(|id| {
                            (
                                *id,
                                create_interact(
                                    *id,
                                    o.interact_shape as u16,
                                    InteractScale::from_slice(&o.interact_scale).unwrap(),
                                    &o.interact_name.clone().unwrap_or_default(),
                                    phashmap![],
                                ),
                            )
                        })
                        .collect(),
                ),
            )
            .await;
        }
    }

    // TODO: partial_sync for newly added/removed units
}

struct InteractScale(pub f64, pub f64, pub f64, pub f64, pub f64);
impl InteractScale {
    pub fn from_slice(v: &[f64]) -> Result<Self> {
        if v.len() != 5 {
            bail!("InteractScale slice should contain 5 values");
        }

        Ok(Self(v[0], v[1], v[2], v[3], v[4]))
    }
}

fn create_interact(
    interact_id: i32,
    interact_shape: u16,
    scale: InteractScale,
    name: &str,
    participators: PropertyHashMap<i32, String>,
) -> InteractInfo {
    InteractInfo {
        interact_id,
        interact_shape,
        scale_x: scale.0,
        scale_y: scale.1,
        scale_z: scale.2,
        scale_w: scale.3,
        scale_r: scale.4,
        name: name.to_string(),
        participators,
    }
}
