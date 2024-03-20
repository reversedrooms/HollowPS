use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

use crate::game::data;
use atomic_refcell::AtomicRefCell;
use protocol::*;
use qwer::{phashmap, phashset, PropertyHashMap, PropertyHashSet};

pub struct HollowGridManager {
    player: Arc<AtomicRefCell<PlayerInfo>>,
    map: AtomicRefCell<Option<HollowGridMapProtocolInfo>>,
    events: AtomicRefCell<HashMap<u64, EventInfo>>,
}

impl HollowGridManager {
    pub fn new(player: Arc<AtomicRefCell<PlayerInfo>>) -> Self {
        Self {
            player,
            map: AtomicRefCell::new(None),
            events: AtomicRefCell::new(HashMap::new()),
        }
    }

    pub fn get_cur_position_in_hollow(&self) -> u16 {
        self.map.borrow().as_ref().unwrap().start_grid
    }

    pub fn move_to(
        &self,
        destination_grid: u16,
        scene_uid: u64,
    ) -> (PtcHollowGridArg, Option<PtcSyncHollowEventInfoArg>) {
        let mut map = self.map.borrow_mut();
        let map = map.as_mut().unwrap();

        map.start_grid = destination_grid;
        let grid = map.grids.get_mut(&destination_grid).unwrap();

        self.update_position_to_scene(scene_uid, destination_grid);

        let mut events = self.events.borrow_mut();
        let sync_event_info =
            if let Entry::Vacant(entry) = events.entry(u64::from(destination_grid)) {
                let event_info = EventInfo {
                    id: 1000,
                    cur_action_id: 1001,
                    action_move_path: vec![1001],
                    state: EventState::WaitingClient,
                    prev_state: EventState::Running,
                    cur_action_info: ActionInfo::None {},
                    cur_action_state: ActionState::Init,
                    predicated_failed_actions: phashset![],
                    stack_frames: Vec::new(),
                };

                entry.insert(event_info.clone());

                Some(PtcSyncHollowEventInfoArg {
                    event_graph_uid: u64::from(destination_grid),
                    hollow_event_template_id: grid.grid.event_graph_info.hollow_event_template_id,
                    event_graph_id: grid.grid.event_graph_info.hollow_event_template_id,
                    updated_event: event_info,
                    specials: phashmap![],
                })
            } else {
                None
            };

        if !grid.grid.event_graph_info.finished {
            grid.grid.flag |= HollowGridFlag::Travelled as i32;
            grid.grid.flag |= HollowGridFlag::ShowEventID as i32;
            grid.grid.flag &= !(HollowGridFlag::Guide as i32);
            grid.grid.flag &= !(HollowGridFlag::CanTriggerEvent as i32);
            grid.grid.flag &= !(HollowGridFlag::ShowEventType as i32);
            grid.grid.event_graph_info.finished = true;

            grid.grid.event_graph_info.fired_count = 2;
        }

        (
            PtcHollowGridArg {
                player_uid: self.player.borrow().uid.unwrap(),
                is_partial: true,
                scene_uid,
                hollow_level: 1,
                grids: HashMap::from([(destination_grid, grid.clone())]),
            },
            sync_event_info,
        )
    }

    pub fn battle_finished(&self) -> (PtcSyncHollowEventInfoArg, bool) {
        let map = self.map.borrow();
        let map = map.as_ref().unwrap();
        let cur_grid = map.grids.get(&map.start_grid).unwrap();

        let event_config =
            data::get_event_config_json(cur_grid.grid.event_graph_info.hollow_event_template_id);

        let mut hollow_finished = false;
        let actions = event_config["Events"]["OnEnd"]["Actions"]
            .as_array()
            .unwrap();
        if let Some(action) = actions.first() {
            hollow_finished = action["$type"].as_str().unwrap() == "Share.CConfigFinishHollow";
        }

        (
            PtcSyncHollowEventInfoArg {
                event_graph_uid: u64::from(map.start_grid),
                hollow_event_template_id: cur_grid.grid.event_graph_info.hollow_event_template_id,
                event_graph_id: cur_grid.grid.event_graph_info.hollow_event_template_id,
                updated_event: EventInfo {
                    id: 1000,
                    cur_action_id: 2001,
                    action_move_path: vec![1001, 1002, 2001],
                    state: EventState::WaitingClient,
                    prev_state: EventState::Running,
                    cur_action_info: ActionInfo::None {},
                    cur_action_state: ActionState::Init,
                    predicated_failed_actions: phashset![],
                    stack_frames: Vec::new(),
                },
                specials: phashmap![],
            },
            hollow_finished,
        )
    }

    pub fn get_cur_event_template_id(&self) -> i32 {
        let map = self.map.borrow();
        let map = map.as_ref().unwrap();
        let cur_grid = map.grids.get(&map.start_grid).unwrap();

        cur_grid.grid.event_graph_info.hollow_event_template_id
    }

    fn update_position_to_scene(&self, scene_uid: u64, pos: u16) {
        let mut player = self.player.borrow_mut();
        let scene = player
            .dungeon_collection
            .as_mut()
            .unwrap()
            .scenes
            .as_mut()
            .unwrap()
            .get_mut(&scene_uid)
            .unwrap();

        if let SceneInfo::Hollow {
            sections_info,
            hollow_event_graph_uid,
            ..
        } = scene
        {
            let section = sections_info.get_mut(&1).unwrap();
            section.prev_grid_index = section.cur_grid_index;
            section.pos_before_move = section.cur_grid_index;
            section.cur_grid_index = pos;
            *hollow_event_graph_uid = u64::from(pos);
        } else {
            panic!("Unexpected scene type")
        }
    }

    pub fn run_event_graph(
        &self,
        event_graph_uid: u64,
        _event_id: i32,
        move_path: Vec<i32>,
    ) -> (
        PtcSyncHollowEventInfoArg,
        PtcHollowGridArg,
        Option<i32>,
        bool,
    ) {
        let (player_uid, scene_uid) = {
            let player = self.player.borrow();

            (player.uid.unwrap(), player.scene_uid.unwrap())
        };

        let mut map = self.map.borrow_mut();
        let map = map.as_mut().unwrap();

        let mut trigger_battle_id = None;
        let mut grid_update = PtcHollowGridArg {
            player_uid,
            is_partial: true,
            scene_uid,
            hollow_level: 1,
            grids: HashMap::new(),
        };

        let mut hollow_finished = false;

        let sync_hollow_event = {
            let info = map.grids.get(&(event_graph_uid as u16)).unwrap().clone();
            let event_config =
                data::get_event_config_json(info.grid.event_graph_info.hollow_event_template_id);

            let mut last_exec_type = "";
            for id in &move_path {
                let index = (id % 1000) - 1;
                let actions = if id / 1000 == 1 {
                    event_config["Events"]["OnStart"]["Actions"]
                        .as_array()
                        .unwrap()
                } else {
                    event_config["Events"]["OnEnd"]["Actions"]
                        .as_array()
                        .unwrap()
                };
                if let Some(action) = actions.get(index as usize) {
                    last_exec_type = action["$type"].as_str().unwrap();

                    match action["$type"].as_str().unwrap() {
                        "Share.CConfigSetMapState" => {
                            let x = action["X"].as_i64().unwrap() as u16;
                            let y = action["Y"].as_i64().unwrap() as u16;

                            let uid = (y * 11) + x;
                            if let Some(info) = map.grids.get_mut(&uid) {
                                info.grid.flag |= HollowGridFlag::Visible as i32
                                    | HollowGridFlag::CanMove as i32
                                    | HollowGridFlag::ShowEventType as i32;

                                grid_update.grids.insert(uid, info.clone());
                            }
                        }
                        "Share.CConfigTriggerBattle" => {
                            trigger_battle_id =
                                Some(match info.grid.event_graph_info.hollow_event_template_id {
                                    1000107 => 10101002,
                                    _ => 10101001,
                                });
                        }
                        "Share.CConfigFinishHollow" => {
                            hollow_finished = true;
                        }
                        _ => {}
                    };
                }
            }

            let mut action_move_path = move_path;

            let last_client_action = *action_move_path.last().unwrap();
            let actions = if last_client_action / 1000 == 1 {
                event_config["Events"]["OnStart"]["Actions"]
                    .as_array()
                    .unwrap()
            } else {
                event_config["Events"]["OnEnd"]["Actions"]
                    .as_array()
                    .unwrap()
            };
            let state = if last_client_action == -1 {
                EventState::Finished
            } else if last_client_action % 1000 >= actions.len() as i32 {
                action_move_path.push(-1);
                EventState::Finished
            } else {
                if last_exec_type != "Share.CConfigEmpty" {
                    action_move_path.push(last_client_action + 1);
                }

                EventState::WaitingClient
            };

            let finish_event = if last_exec_type != "Share.CConfigTriggerBattle" {
                PtcSyncHollowEventInfoArg {
                    event_graph_uid,
                    hollow_event_template_id: info.grid.event_graph_info.hollow_event_template_id,
                    event_graph_id: info.grid.event_graph_info.hollow_event_template_id,
                    updated_event: EventInfo {
                        id: 1000,
                        cur_action_id: *action_move_path.last().unwrap(),
                        action_move_path,
                        state,
                        prev_state: EventState::Running,
                        cur_action_info: ActionInfo::None {},
                        cur_action_state: ActionState::Init,
                        predicated_failed_actions: phashset![],
                        stack_frames: Vec::new(),
                    },
                    specials: phashmap![],
                }
            } else {
                PtcSyncHollowEventInfoArg {
                    event_graph_uid,
                    hollow_event_template_id: info.grid.event_graph_info.hollow_event_template_id,
                    event_graph_id: info.grid.event_graph_info.hollow_event_template_id,
                    updated_event: EventInfo {
                        id: 0,
                        cur_action_id: 0,
                        action_move_path: vec![],
                        state: EventState::Initing,
                        prev_state: EventState::Initing,
                        cur_action_info: ActionInfo::None {},
                        cur_action_state: ActionState::Init,
                        predicated_failed_actions: phashset![],
                        stack_frames: Vec::new(),
                    },
                    specials: phashmap![],
                }
            };

            //tracing::info!("sending evt info: {:#?}", &finish_event);
            finish_event
        };

        (
            sync_hollow_event,
            grid_update,
            trigger_battle_id,
            hollow_finished,
        )
    }

    pub fn sync_hollow_maps(&self, player_uid: u64, scene_uid: u64) -> PtcSyncHollowGridMapsArg {
        PtcSyncHollowGridMapsArg {
            player_uid,
            scene_uid,
            hollow_level: 1,
            main_map: self.map.borrow().clone().unwrap(),
            time_period: TimePeriodType::Random,
            weather: WeatherType::Random,
        }
    }

    pub fn init_default_map(&self) {
        *self.map.borrow_mut() = Some(HollowGridMapProtocolInfo {
            row: 5,
            col: 11,
            start_grid: 22,
            grids: phashmap![
                (
                    48,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 2848,
                            link_to: 12,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1000,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::DialogPositive,
                        use_perform: false,
                    }
                ),
                (
                    7,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 2872,
                            link_to: 10,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1017,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::DialogPositive,
                        use_perform: false,
                    }
                ),
                (
                    24,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 2658,
                            link_to: 12,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1000,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::DialogPositive,
                        use_perform: false,
                    }
                ),
                (
                    36,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 2848,
                            link_to: 3,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1000,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::DialogPositive,
                        use_perform: false,
                    }
                ),
                (
                    29,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 2848,
                            link_to: 5,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1000,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::DialogPositive,
                        use_perform: false,
                    }
                ),
                (
                    49,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 2872,
                            link_to: 9,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1018,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::DialogPositive,
                        use_perform: false,
                    }
                ),
                (
                    27,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 2848,
                            link_to: 3,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1000104,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::Dialog,
                        use_perform: false,
                    }
                ),
                (
                    6,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 2848,
                            link_to: 12,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1000,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::DialogPositive,
                        use_perform: false,
                    }
                ),
                (
                    16,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 2848,
                            link_to: 3,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1000105,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::Dialog,
                        use_perform: false,
                    }
                ),
                (
                    22,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 2686,
                            link_to: 4,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1000101,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::Begin,
                        use_perform: false,
                    }
                ),
                (
                    30,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 2848,
                            link_to: 12,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1000,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::DialogPositive,
                        use_perform: false,
                    }
                ),
                (
                    18,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 2848,
                            link_to: 3,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1000,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::DialogPositive,
                        use_perform: false,
                    }
                ),
                (
                    38,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 2848,
                            link_to: 3,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1000,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::DialogPositive,
                        use_perform: false,
                    }
                ),
                (
                    32,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 2872,
                            link_to: 8,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1000107,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::ChangeLevelInteract,
                        use_perform: false,
                    }
                ),
                (
                    47,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 2872,
                            link_to: 5,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1000103,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::BattleNormal,
                        use_perform: false,
                    }
                ),
                (
                    25,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 2682,
                            link_to: 10,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1000102,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::Dialog,
                        use_perform: false,
                    }
                ),
                (
                    5,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 2848,
                            link_to: 6,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1000,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::DialogPositive,
                        use_perform: false,
                    }
                ),
                (
                    31,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 2848,
                            link_to: 12,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1000106,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::Dialog,
                        use_perform: false,
                    }
                ),
                (
                    23,
                    HollowGridProtocolInfo {
                        grid: HollowGridInfo {
                            flag: 35434,
                            link_to: 12,
                            event_graph_info: HollowEventGraphInfo {
                                config_id: 0,
                                events_info: phashmap![],
                                specials: phashmap![],
                                is_new: false,
                                finished: false,
                                list_specials: phashmap![],
                                fired_count: 0,
                                hollow_event_template_id: 1000109,
                                uid: 0,
                                is_create_by_gm: false,
                            },
                            travelled_count: 0,
                            node_state: NodeState::All,
                            node_visible: NodeVisible::All,
                        },
                        event_type: HollowEventType::Dialog,
                        use_perform: false,
                    }
                )
            ],
            chessboard_id: 1000101,
        });
    }
}
