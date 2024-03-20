use std::{collections::HashMap, sync::Arc};

use atomic_refcell::AtomicRefCell;
use protocol::*;
use qwer::{phashmap, PropertyHashMap};

use super::UniqueIDManager;

pub struct SceneUnitManager {
    uid_mgr: Arc<AtomicRefCell<UniqueIDManager>>,
    units: AtomicRefCell<HashMap<u64, SceneUnitProtocolInfo>>,
}

impl SceneUnitManager {
    pub fn new(uid_mgr: Arc<AtomicRefCell<UniqueIDManager>>) -> Self {
        Self {
            uid_mgr,
            units: AtomicRefCell::new(HashMap::new()),
        }
    }

    pub fn create_npc(
        &self,
        id: i32,
        tag: i32,
        quest_id: i32,
        interacts_info: PropertyHashMap<i32, InteractInfo>,
    ) -> u64 {
        let uid = self.uid_mgr.borrow().next();

        self.units.borrow_mut().insert(
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

    pub fn get(&self, uid: u64) -> SceneUnitProtocolInfo {
        self.units.borrow().get(&uid).unwrap().clone()
    }

    pub fn sync(&self, scene_uid: u64, section_id: i32) -> PtcSyncSceneUnitArg {
        PtcSyncSceneUnitArg {
            scene_uid,
            section_id,
            is_partial: false,
            removed_scene_units: Vec::new(),
            scene_units: self
                .units
                .borrow()
                .iter()
                .map(|(_, unit)| unit.clone())
                .collect(),
        }
    }

    // TODO: partial_sync for newly added/removed units

    // currently hardcoded for Main City section 2
    pub fn add_default_units(&self) {
        self.create_npc(
            100171011,
            3,
            0,
            phashmap![(
                19900006,
                create_interact(
                    0,
                    1,
                    2.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    "",
                    phashmap![(0, String::new())]
                )
            )],
        );

        self.create_npc(
            100171011,
            4,
            0,
            phashmap![(
                19900006,
                create_interact(
                    0,
                    1,
                    2.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    "",
                    phashmap![(0, String::new())]
                )
            )],
        );

        self.create_npc(
            100171011,
            1002,
            0,
            phashmap![(
                19900062,
                create_interact(
                    0,
                    1,
                    2.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    "",
                    phashmap![(0, String::new())]
                )
            )],
        );

        self.create_npc(
            100171011,
            1001,
            0,
            phashmap![(
                10000010,
                create_interact(
                    10000010,
                    1,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    "A",
                    phashmap![(1001, String::from("A"))]
                )
            )],
        );

        self.create_npc(
            100171011,
            1005,
            0,
            phashmap![(
                10000014,
                create_interact(
                    10000014,
                    1,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    "A",
                    phashmap![(1005, String::from("A"))]
                )
            )],
        );

        self.create_npc(
            100173001,
            2028,
            0,
            phashmap![(
                19900052,
                create_interact(
                    19900052,
                    2,
                    9.0,
                    2.0,
                    2.0,
                    90.0,
                    10.0,
                    "A",
                    phashmap![(2028, String::from("A"))]
                )
            )],
        );

        self.create_npc(
            100172011,
            2000,
            0,
            phashmap![(
                19900030,
                create_interact(
                    0,
                    1,
                    2.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    "",
                    phashmap![(2000, String::from("A")), (2052, String::from("B"))]
                )
            )],
        );

        self.create_npc(100172081, 2052, 0, phashmap![]);
    }
}

#[allow(clippy::too_many_arguments)]
fn create_interact(
    interact_id: i32,
    interact_shape: u16,
    scale_x: f64,
    scale_y: f64,
    scale_z: f64,
    scale_w: f64,
    scale_r: f64,
    name: &str,
    participators: PropertyHashMap<i32, String>,
) -> InteractInfo {
    InteractInfo {
        interact_id,
        interact_shape,
        scale_x,
        scale_y,
        scale_z,
        scale_w,
        scale_r,
        name: name.to_string(),
        participators,
    }
}
