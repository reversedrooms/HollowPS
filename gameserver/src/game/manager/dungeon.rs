use anyhow::{anyhow, bail, Result};
use atomic_refcell::AtomicRefCell;
use protocol::*;
use std::sync::Arc;

use crate::game::{manager::UniqueIDManager, util, PlayerOperationResult};
use qwer::{
    pdkhashmap, phashmap, phashset, PropertyDoubleKeyHashMap, PropertyHashMap, PropertyHashSet,
};

pub struct DungeonManager {
    uid_mgr: Arc<AtomicRefCell<UniqueIDManager>>,
    player: Arc<AtomicRefCell<PlayerInfo>>,
    scene_properties: AtomicRefCell<PropertyDoubleKeyHashMap<u64, u16, i32>>,
}

impl DungeonManager {
    pub fn new(
        uid_mgr: Arc<AtomicRefCell<UniqueIDManager>>,
        player: Arc<AtomicRefCell<PlayerInfo>>,
    ) -> Self {
        Self {
            uid_mgr,
            player,
            scene_properties: AtomicRefCell::new(pdkhashmap![]),
        }
    }

    pub fn enter_main_city(&self) -> Result<PlayerOperationResult<PtcEnterSceneArg>> {
        let (player_uid, scene_position, scene_rotation) = {
            let player = self.player.borrow();
            let pos_in_main_city = player.pos_in_main_city.as_ref().unwrap();

            (
                player.uid.unwrap(),
                *pos_in_main_city.position.as_ref().unwrap(),
                *pos_in_main_city.rotation.as_ref().unwrap(),
            )
        };

        let mut player = self.player.borrow_mut();
        let default_scene_uid = player
            .dungeon_collection
            .as_ref()
            .unwrap()
            .default_scene_uid
            .unwrap();

        player.scene_uid.replace(default_scene_uid);
        let dungeon_collection = player.dungeon_collection.as_mut().unwrap();

        let scene_info = dungeon_collection
            .scenes
            .as_mut()
            .unwrap()
            .get_mut(&default_scene_uid)
            .ok_or_else(|| anyhow!("Scene with uid {default_scene_uid} doesn't exist"))?;

        let dungeon_uid = scene_info.get_dungeon_uid();
        let dungeon_info = dungeon_collection
            .dungeons
            .as_mut()
            .unwrap()
            .get_mut(&scene_info.get_dungeon_uid())
            .ok_or_else(|| anyhow!("Dungeon with uid {dungeon_uid} doesn't exist"))?;

        scene_info.set_entered_times(scene_info.get_entered_times() + 1);
        dungeon_info.entered_times += 1;

        let ptc_enter_scene = PtcEnterSceneArg {
            player_uid,
            scene_uid: default_scene_uid,
            section_id: scene_info.get_section_id(),
            open_ui: UIType::Default,
            condition_config_ids: Vec::new(),
            transform: Transform {
                position: scene_position,
                rotation: scene_rotation,
            },
            timestamp: util::cur_timestamp_ms(),
            camera_x: 0,
            camera_y: 6000,
            entered_times: scene_info.get_entered_times(),
            ext: match scene_info {
                SceneInfo::Hall { .. } => SceneTableExt::Hall {
                    event_graphs_info: EventGraphsInfo {
                        default_event_graph_id: 0,
                        event_graphs_info: phashmap![],
                    },
                },
                _ => bail!("Unexpected main city scene type"),
            },
        };

        Ok(PlayerOperationResult::with_changes(
            ptc_enter_scene,
            PlayerInfo {
                dungeon_collection: Some(DungeonCollection {
                    dungeons: Some(PropertyHashMap::Modify {
                        to_add: vec![(dungeon_info.uid, dungeon_info.clone())],
                        to_remove: Vec::new(),
                    }),
                    scenes: Some(PropertyHashMap::Modify {
                        to_add: vec![(scene_info.get_uid(), scene_info.clone())],
                        to_remove: Vec::new(),
                    }),
                    ..Default::default()
                }),
                scene_uid: Some(scene_info.get_uid()),
                ..Default::default()
            },
        ))
    }

    pub fn enter_scene_section(
        &self,
        scene_uid: u64,
        section_id: i32,
    ) -> PlayerOperationResult<PtcEnterSectionArg> {
        let mut player = self.player.borrow_mut();
        let scene_info = player
            .dungeon_collection
            .as_mut()
            .unwrap()
            .scenes
            .as_mut()
            .unwrap()
            .get_mut(&scene_uid)
            .unwrap();
        scene_info.set_section_id(section_id);

        PlayerOperationResult::with_changes(
            PtcEnterSectionArg { section_id },
            PlayerInfo {
                dungeon_collection: Some(DungeonCollection {
                    scenes: Some(PropertyHashMap::Modify {
                        to_add: vec![(scene_uid, scene_info.clone())],
                        to_remove: Vec::new(),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
    }

    pub fn enter_scene(&self, scene_uid: u64) -> Result<PlayerOperationResult<PtcEnterSceneArg>> {
        let (player_uid, prev_scene_uid) = {
            let player = self.player.borrow();

            (
                *player.uid.as_ref().unwrap(),
                *player.scene_uid.as_ref().unwrap(),
            )
        };

        let mut player = self.player.borrow_mut();
        player.scene_uid.replace(scene_uid);
        player.prev_scene_uid.replace(prev_scene_uid);
        let dungeon_collection = player.dungeon_collection.as_mut().unwrap();

        let scene_info = dungeon_collection
            .scenes
            .as_mut()
            .unwrap()
            .get_mut(&scene_uid)
            .ok_or_else(|| anyhow!("Scene with uid {scene_uid} doesn't exist"))?;

        let dungeon_uid = scene_info.get_dungeon_uid();
        let dungeon_info = dungeon_collection
            .dungeons
            .as_mut()
            .unwrap()
            .get_mut(&scene_info.get_dungeon_uid())
            .ok_or_else(|| anyhow!("Dungeon with uid {dungeon_uid} doesn't exist"))?;

        scene_info.set_entered_times(scene_info.get_entered_times() + 1);
        dungeon_info.entered_times += 1;

        if let SceneInfo::Hollow { sections_info, .. } = scene_info {
            let section = sections_info.get_mut(&1).unwrap();
            section.entered_times += 1;
        }

        let ptc_enter_scene = PtcEnterSceneArg {
            player_uid,
            scene_uid,
            section_id: scene_info.get_section_id(),
            open_ui: UIType::Default,
            condition_config_ids: Vec::new(),
            transform: Transform::default(),
            timestamp: util::cur_timestamp_ms(),
            camera_x: 0,
            camera_y: 6000,
            entered_times: scene_info.get_entered_times(),
            ext: match scene_info {
                SceneInfo::Hall { .. } => SceneTableExt::Hall {
                    event_graphs_info: EventGraphsInfo {
                        default_event_graph_id: 0,
                        event_graphs_info: phashmap![],
                    },
                },
                SceneInfo::Fresh { .. } => SceneTableExt::Fresh {
                    event_graphs_info: EventGraphsInfo {
                        default_event_graph_id: 0,
                        event_graphs_info: phashmap![],
                    },
                },
                SceneInfo::Hollow { .. } => SceneTableExt::Hollow {
                    event_graphs_info: EventGraphsInfo {
                        default_event_graph_id: 0,
                        event_graphs_info: phashmap![],
                    },
                    grid_random_seed: 0,
                    alter_section_id: 0,
                },
                SceneInfo::Fight { .. } => SceneTableExt::Fight {
                    event_graphs_info: EventGraphsInfo {
                        default_event_graph_id: 0,
                        event_graphs_info: phashmap![],
                    },
                },
            },
        };

        Ok(PlayerOperationResult::with_changes(
            ptc_enter_scene,
            PlayerInfo {
                dungeon_collection: Some(DungeonCollection {
                    dungeons: Some(PropertyHashMap::Modify {
                        to_add: vec![(dungeon_info.uid, dungeon_info.clone())],
                        to_remove: Vec::new(),
                    }),
                    scenes: Some(PropertyHashMap::Modify {
                        to_add: vec![(scene_info.get_uid(), scene_info.clone())],
                        to_remove: Vec::new(),
                    }),
                    ..Default::default()
                }),
                scene_uid: Some(scene_uid),
                prev_scene_uid: Some(prev_scene_uid),
                ..Default::default()
            },
        ))
    }

    pub fn hollow_finished(&self) -> PlayerOperationResult<u64> {
        let cur_scene_uid = self.get_cur_scene_uid();

        let mut player = self.player.borrow_mut();

        let hollow_scene = player
            .dungeon_collection
            .as_mut()
            .unwrap()
            .scenes
            .as_mut()
            .unwrap()
            .get_mut(&cur_scene_uid)
            .unwrap();

        if let SceneInfo::Hollow {
            hollow_system_ui_state,
            ..
        } = hollow_scene
        {
            hollow_system_ui_state.insert(
                HollowSystemType::HollowResultPage,
                HollowSystemUIState::Normal,
            );
            hollow_system_ui_state.insert(HollowSystemType::Menu, HollowSystemUIState::Close);
        }

        PlayerOperationResult::with_changes(
            cur_scene_uid,
            PlayerInfo {
                dungeon_collection: Some(DungeonCollection {
                    scenes: Some(PropertyHashMap::Modify {
                        to_add: vec![(cur_scene_uid, hollow_scene.clone())],
                        to_remove: Vec::new(),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
    }

    pub fn get_default_scene_uid(&self) -> u64 {
        self.player
            .borrow()
            .dungeon_collection
            .as_ref()
            .unwrap()
            .default_scene_uid
            .unwrap()
    }

    #[allow(dead_code)]
    pub fn get_default_scene_uid_for_dungeon(&self, dungeon_uid: u64) -> u64 {
        self.player
            .borrow()
            .dungeon_collection
            .as_ref()
            .unwrap()
            .dungeons
            .as_ref()
            .unwrap()
            .get(&dungeon_uid)
            .unwrap()
            .default_scene_uid
    }

    pub fn get_cur_scene_uid(&self) -> u64 {
        self.player.borrow().scene_uid.unwrap()
    }

    fn add_default_hollow_properties(&self, scene_uid: u64) {
        let mut props = self.scene_properties.borrow_mut();

        for (sub_key, value) in &[
            (1001, 0),
            (1002, 100),
            (1003, 0),
            (1004, 0),
            (1019, 10),
            (1020, 1),
            (1005, 0),
            (1006, 0),
            (1007, 0),
            (1008, 0),
            (1009, 0),
            (1010, 0),
            (1011, 0),
            (1012, 0),
            (1013, 1),
            (1014, 1),
            (1015, 0),
            (1016, 0),
            (1017, 4),
            (1018, 10000),
            (1021, 1),
            (1025, 1),
            (1035, 10000),
            (1041, 10000),
            (1042, 10000),
            (1043, 1),
            (1044, 1),
        ] {
            props.insert(scene_uid, *sub_key, *value);
        }
    }

    pub fn leave_battle(&self) -> Result<PlayerOperationResult<PtcEnterSceneArg>> {
        let back_scene_uid = self.get_back_scene_uid();

        {
            let mut player = self.player.borrow_mut();

            let hollow_scene = player
                .dungeon_collection
                .as_mut()
                .unwrap()
                .scenes
                .as_mut()
                .unwrap()
                .get_mut(&back_scene_uid)
                .unwrap();

            if let SceneInfo::Hollow {
                battle_scene_uid, ..
            } = hollow_scene
            {
                *battle_scene_uid = 0;
            }
        }

        self.enter_scene(back_scene_uid)
    }

    fn get_back_scene_uid(&self) -> u64 {
        let player = self.player.borrow();
        let fight_scene_uid = player.scene_uid.as_ref().unwrap();
        let fight_scene = player
            .dungeon_collection
            .as_ref()
            .unwrap()
            .scenes
            .as_ref()
            .unwrap()
            .get(fight_scene_uid)
            .unwrap();

        fight_scene.get_back_scene_uid()
    }

    pub fn enter_battle(&self, scene_uid: u64) -> PlayerOperationResult<PtcEnterSceneArg> {
        let hollow_scene_uid = *self.player.borrow().scene_uid.as_ref().unwrap();
        let hollow_scene = self.set_cur_hollow_battle(scene_uid, hollow_scene_uid);
        let ptc_enter_scene = self.enter_scene(scene_uid).unwrap().unwrap().clone();

        let player = self.player.borrow();
        let dungeon_collection = player.dungeon_collection.as_ref().unwrap();
        let fight_scene = dungeon_collection
            .scenes
            .as_ref()
            .unwrap()
            .get(&scene_uid)
            .unwrap();

        PlayerOperationResult::with_changes(
            ptc_enter_scene,
            PlayerInfo {
                dungeon_collection: Some(DungeonCollection {
                    scenes: Some(PropertyHashMap::Modify {
                        to_add: vec![
                            (hollow_scene_uid, hollow_scene),
                            (scene_uid, fight_scene.clone()),
                        ],
                        to_remove: Vec::new(),
                    }),
                    ..Default::default()
                }),
                scene_uid: Some(scene_uid),
                prev_scene_uid: Some(hollow_scene_uid),
                ..Default::default()
            },
        )
    }

    fn set_cur_hollow_battle(&self, scene_uid: u64, hollow_scene_uid: u64) -> SceneInfo {
        let mut player = self.player.borrow_mut();
        let hollow_scene = player
            .dungeon_collection
            .as_mut()
            .unwrap()
            .scenes
            .as_mut()
            .unwrap()
            .get_mut(&hollow_scene_uid)
            .unwrap();
        let SceneInfo::Hollow {
            on_battle_success,
            battle_scene_uid,
            ..
        } = hollow_scene
        else {
            panic!("Unexpected scene type")
        };

        *battle_scene_uid = scene_uid;
        *on_battle_success = String::from("OnEnd");

        hollow_scene.clone()
    }

    pub fn create_fight(&self, id: i32, hollow_scene_uid: u64) -> PlayerOperationResult<u64> {
        let mut player = self.player.borrow_mut();
        let dungeon_collection = player.dungeon_collection.as_mut().unwrap();
        let scenes = dungeon_collection.scenes.as_mut().unwrap();
        let hollow_scene = scenes.get_mut(&hollow_scene_uid).unwrap();

        let fight_scene_uid = self.uid_mgr.borrow().next();
        let fight_scene = SceneInfo::Fight {
            uid: fight_scene_uid,
            id,
            dungeon_uid: hollow_scene.get_dungeon_uid(),
            end_timestamp: 0,
            back_scene_uid: hollow_scene_uid,
            entered_times: 1,
            section_id: 1,
            open_ui: UIType::Default,
            to_be_destroyed: false,
            camera_x: 0xFFFFFFFF,
            camera_y: 0xFFFFFFFF,
            perform_show_progress: phashmap![],
            end_hollow: false,
            random_seed: 2281337,
        };

        scenes.insert(fight_scene_uid, fight_scene.clone());
        PlayerOperationResult::with_changes(
            fight_scene_uid,
            PlayerInfo {
                dungeon_collection: Some(DungeonCollection {
                    scenes: Some(PropertyHashMap::Modify {
                        to_add: vec![(fight_scene_uid, fight_scene)],
                        to_remove: Vec::new(),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
    }

    pub fn is_in_tutorial(&self) -> bool {
        let cur_scene_uid = self.get_cur_scene_uid();

        let player = self.player.borrow();
        let cur_scene = player
            .dungeon_collection
            .as_ref()
            .unwrap()
            .scenes
            .as_ref()
            .unwrap()
            .get(&cur_scene_uid)
            .unwrap();

        matches!(cur_scene, SceneInfo::Fresh { .. })
    }

    pub fn create_hollow(
        &self,
        id: i32,
        world_quest_id: i32,
        avatar_uids: &[u64],
    ) -> PlayerOperationResult<(u64, u64)> {
        let back_scene_uid = self.get_default_scene_uid();

        let mut dungeon = self.create_base_dungeon(id, back_scene_uid, world_quest_id);
        dungeon.hollow_event_version = 526;

        let scene_uid = self.uid_mgr.borrow().next();
        dungeon.default_scene_uid = scene_uid;
        dungeon.scene_properties_uid = scene_uid;

        self.add_default_hollow_properties(scene_uid);

        for (index, avatar_uid) in avatar_uids.iter().enumerate() {
            dungeon.avatar_map.insert(
                index.try_into().unwrap(),
                AvatarUnitInfo {
                    uid: *avatar_uid,
                    properties_uid: self.uid_mgr.borrow().next(),
                    is_banned: false,
                    modified_property: pdkhashmap![],
                    hp_add_hollow: 0,
                    hp_lost_hollow: 0,
                    layer_property_change: phashmap![],
                },
            );
        }

        let scene = SceneInfo::Hollow {
            uid: scene_uid,
            id,
            dungeon_uid: dungeon.uid,
            end_timestamp: 0,
            back_scene_uid,
            entered_times: 1,
            section_id: 1,
            open_ui: UIType::Default,
            to_be_destroyed: false,
            camera_x: 0xFFFFFFFF,
            camera_y: 0xFFFFFFFF,
            event_variables: phashmap![],
            buddy: BuddyUnitInfo {
                uid: 0,
                properties: 0,
            },
            stress_punish_ability_random_pool: vec![String::from(
                "Stress_Punish_RandomDebuff_Normal",
            )],
            finished: false,
            event_weight_factor: phashmap![],
            shop_modification: HollowShopModification {
                ability_modified_num: pdkhashmap![],
                action_modified_num: phashmap![],
                overwrite_price: phashmap![],
            },
            last_challenge_stat: phashmap![],
            cur_challenge: phashset![],
            hollow_system_switch: phashmap![],
            sections_info: phashmap![(
                1,
                PlayerHollowSectionInfo {
                    prev_grid_index: 0,
                    cur_grid_index: 22,
                    entered_times: 0,
                    global_event: 0,
                    perform_event_graph: 3405096459205834,
                    pos_before_move: 0,
                }
            )],
            executing_event: true,
            event_id: 1000,
            hollow_event_graph_uid: 22,
            on_battle_success: String::new(),
            on_battle_failure: String::new(),
            battle_finished: false,
            battle_success: false,
            battle_scene_uid: 0,
            scene_global_events: phashmap![],
            prepare_section: PrepareSection {
                section_id: 0,
                initial_pos: 0,
                show_other: false,
                battle_end_goto_next_hollow: false,
            },
            abilities_info: AbilitiesInfo {
                abilities: phashmap![],
                sequence_no: 0,
            },
            blackout: false,
            hollow_system_ui_state: phashmap![],
        };

        {
            let mut player = self.player.borrow_mut();
            player
                .scene_properties
                .replace(self.scene_properties.borrow().clone());

            let dungeon_collection = player.dungeon_collection.as_mut().unwrap();

            dungeon_collection
                .dungeons
                .as_mut()
                .unwrap()
                .insert(dungeon.uid, dungeon.clone());
            dungeon_collection
                .scenes
                .as_mut()
                .unwrap()
                .insert(scene_uid, scene.clone());
        }
        let mut player = self.player.borrow_mut();
        let items = player.items.as_mut().unwrap();

        let mut updated_items = Vec::new();
        for avatar_uid in avatar_uids {
            let item = items.get_mut(avatar_uid).unwrap();
            let ItemInfo::Avatar { robot_id, .. } = item else {
                continue;
            };
            *robot_id = 101000101;
            updated_items.push((*avatar_uid, item.clone()));
        }

        let mut prop_changes = Vec::new();
        for (key, sub_key, value) in &*self.scene_properties.borrow_mut() {
            prop_changes.push((*key, *sub_key, *value));
        }

        PlayerOperationResult::with_changes(
            (dungeon.uid, scene_uid),
            PlayerInfo {
                items: Some(PropertyHashMap::Modify {
                    to_add: updated_items,
                    to_remove: vec![],
                }),
                dungeon_collection: Some(DungeonCollection {
                    dungeons: Some(PropertyHashMap::Modify {
                        to_add: vec![(dungeon.uid, dungeon)],
                        to_remove: Vec::new(),
                    }),
                    scenes: Some(PropertyHashMap::Modify {
                        to_add: vec![(scene_uid, scene)],
                        to_remove: Vec::new(),
                    }),
                    ..Default::default()
                }),
                scene_properties: Some(PropertyDoubleKeyHashMap::Modify {
                    to_add: prop_changes,
                    to_remove: Vec::new(),
                }),
                ..Default::default()
            },
        )
    }

    pub fn create_hall(&self, id: i32) -> PlayerOperationResult<u64> {
        let mut dungeon = self.create_base_dungeon(id, 0, 0);
        let dungeon_uid = dungeon.uid;

        let scene_uid = self.uid_mgr.borrow().next();
        let hall_scene_info = SceneInfo::Hall {
            uid: scene_uid,
            id,
            dungeon_uid,
            end_timestamp: 0,
            back_scene_uid: 0,
            entered_times: 1,
            section_id: 1,
            open_ui: UIType::Default,
            to_be_destroyed: true,
            camera_x: 0xFFFFFFFF,
            camera_y: 0xFFFFFFFF,
        };

        dungeon.default_scene_uid = scene_uid;

        let mut player = self.player.borrow_mut();
        let dungeon_collection = player.dungeon_collection.as_mut().unwrap();

        dungeon_collection
            .dungeons
            .as_mut()
            .unwrap()
            .insert(dungeon_uid, dungeon.clone());

        dungeon_collection
            .scenes
            .as_mut()
            .unwrap()
            .insert(scene_uid, hall_scene_info.clone());

        dungeon_collection.default_scene_uid.replace(scene_uid);
        PlayerOperationResult::with_changes(
            scene_uid,
            PlayerInfo {
                dungeon_collection: Some(DungeonCollection {
                    dungeons: Some(PropertyHashMap::Modify {
                        to_add: vec![(dungeon_uid, dungeon)],
                        to_remove: vec![],
                    }),
                    scenes: Some(PropertyHashMap::Modify {
                        to_add: vec![(scene_uid, hall_scene_info)],
                        to_remove: vec![],
                    }),
                    default_scene_uid: Some(scene_uid),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
    }

    pub fn create_fresh(&self) -> PlayerOperationResult<u64> {
        let mut dungeon = self.create_base_dungeon(2, 0, 0);
        let dungeon_uid = dungeon.uid;

        let scene_uid = self.uid_mgr.borrow().next();
        let fresh_scene_info = SceneInfo::Fresh {
            uid: scene_uid,
            id: 2,
            dungeon_uid,
            end_timestamp: 0,
            back_scene_uid: 0,
            entered_times: 1,
            section_id: 1,
            open_ui: UIType::Default,
            to_be_destroyed: true,
            camera_x: 0xFFFFFFFF,
            camera_y: 0xFFFFFFFF,
        };

        dungeon.default_scene_uid = scene_uid;

        let mut player = self.player.borrow_mut();
        let dungeon_collection = player.dungeon_collection.as_mut().unwrap();

        dungeon_collection
            .dungeons
            .as_mut()
            .unwrap()
            .insert(dungeon_uid, dungeon.clone());

        dungeon_collection
            .scenes
            .as_mut()
            .unwrap()
            .insert(scene_uid, fresh_scene_info.clone());

        PlayerOperationResult::with_changes(
            scene_uid,
            PlayerInfo {
                dungeon_collection: Some(DungeonCollection {
                    dungeons: Some(PropertyHashMap::Modify {
                        to_add: vec![(dungeon_uid, dungeon)],
                        to_remove: vec![],
                    }),
                    scenes: Some(PropertyHashMap::Modify {
                        to_add: vec![(scene_uid, fresh_scene_info)],
                        to_remove: vec![],
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
    }

    fn create_base_dungeon(
        &self,
        id: i32,
        back_scene_uid: u64,
        world_quest_id: i32,
    ) -> DungeonInfo {
        let player = self.player.borrow();
        let uid = self.uid_mgr.borrow().next();

        DungeonInfo {
            uid,
            id,
            default_scene_uid: 0,
            start_timestamp: util::cur_timestamp_ms(),
            to_be_destroyed: false,
            back_scene_uid,
            quest_collection_uid: uid,
            avatars: phashmap![],
            buddy: BuddyUnitInfo {
                uid: 0,
                properties: 0,
            },
            world_quest_id,
            scene_properties_uid: 0,
            drop_poll_chg_infos: phashmap![],
            is_in_dungeon: false,
            initiative_item: 0,
            initiative_item_used_times: 0,
            avatar_map: phashmap![],
            battle_report: Vec::new(),
            dungeon_group_uid: player.uid.unwrap(),
            entered_times: 0,
            is_preset_avatar: false,
            hollow_event_version: 0,
        }
    }
}
