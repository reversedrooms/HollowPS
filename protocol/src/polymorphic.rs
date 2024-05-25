use super::*;

macro_rules! polymorphic_scene_unit_protocol_info {
    (enum $name:ident { $($variant:ident { $($field:ident: $ty:ty),* $(,)? } = $tag:expr,)* }) => {
        #[derive(OctData, Clone, Debug)]
        #[repr(u16)]
        #[base = 2]
        pub enum $name {
            $(
                $variant {
                    uid: u64,
                    tag: i32,
                    $($field: $ty),*
                } = $tag,
            )*
        }

        impl $name {
            #[must_use]
            pub const fn get_uid(&self) -> u64 {
                match self {
                    $(
                        $name::$variant { uid, .. } => *uid,
                    )*
                }
            }

            pub fn set_uid(&mut self, uid: u64) {
                match self {
                    $(
                        $name::$variant { uid: ref mut u, .. } => *u = uid,
                    )*
                }
            }

            #[must_use]
            pub const fn get_tag(&self) -> i32 {
                match self {
                    $(
                        $name::$variant { tag, .. } => *tag,
                    )*
                }
            }

            pub fn set_tag(&mut self, tag: i32) {
                match self {
                    $(
                        $name::$variant { tag: ref mut t, .. } => *t = tag,
                    )*
                }
            }
        }
    };
}

macro_rules! polymorphic_scene_info {
    (enum $name:ident { $($variant:ident { $($field:ident: $ty:ty),* $(,)? } = $tag:expr,)* }) => {
        #[derive(OctData, Clone, Debug)]
        #[repr(u16)]
        #[base = 11]
        pub enum $name {
            $(
                $variant {
                    uid: u64,
                    id: i32,
                    dungeon_uid: u64,
                    end_timestamp: u64,
                    back_scene_uid: u64,
                    entered_times: u16,
                    section_id: i32,
                    open_ui: UIType,
                    to_be_destroyed: bool,
                    camera_x: u32,
                    camera_y: u32,
                    $($field: $ty),*
                } = $tag,
            )*
        }

        impl $name {
            #[must_use]
            pub const fn get_uid(&self) -> u64 {
                match self {
                    $(
                        $name::$variant { uid, .. } => *uid,
                    )*
                }
            }

            pub fn set_uid(&mut self, uid: u64) {
                match self {
                    $(
                        $name::$variant { uid: ref mut u, .. } => *u = uid,
                    )*
                }
            }

            #[must_use]
            pub const fn get_id(&self) -> i32 {
                match self {
                    $(
                        $name::$variant { id, .. } => *id,
                    )*
                }
            }

            pub fn set_id(&mut self, id: i32) {
                match self {
                    $(
                        $name::$variant { id: ref mut i, .. } => *i = id,
                    )*
                }
            }

            #[must_use]
            pub const fn get_dungeon_uid(&self) -> u64 {
                match self {
                    $(
                        $name::$variant { dungeon_uid, .. } => *dungeon_uid,
                    )*
                }
            }

            pub fn set_dungeon_uid(&mut self, dungeon_uid: u64) {
                match self {
                    $(
                        $name::$variant { dungeon_uid: ref mut d, .. } => *d = dungeon_uid,
                    )*
                }
            }

            #[must_use]
            pub const fn get_end_timestamp(&self) -> u64 {
                match self {
                    $(
                        $name::$variant { end_timestamp, .. } => *end_timestamp,
                    )*
                }
            }

            pub fn set_end_timestamp(&mut self, end_timestamp: u64) {
                match self {
                    $(
                        $name::$variant { end_timestamp: ref mut e, .. } => *e = end_timestamp,
                    )*
                }
            }

            #[must_use]
            pub const fn get_back_scene_uid(&self) -> u64 {
                match self {
                    $(
                        $name::$variant { back_scene_uid, .. } => *back_scene_uid,
                    )*
                }
            }

            pub fn set_back_scene_uid(&mut self, back_scene_uid: u64) {
                match self {
                    $(
                        $name::$variant { back_scene_uid: ref mut b, .. } => *b = back_scene_uid,
                    )*
                }
            }

            #[must_use]
            pub const fn get_entered_times(&self) -> u16 {
                match self {
                    $(
                        $name::$variant { entered_times, .. } => *entered_times,
                    )*
                }
            }

            pub fn set_entered_times(&mut self, entered_times: u16) {
                match self {
                    $(
                        $name::$variant { entered_times: ref mut e, .. } => *e = entered_times,
                    )*
                }
            }

            #[must_use]
            pub const fn get_section_id(&self) -> i32 {
                match self {
                    $(
                        $name::$variant { section_id, .. } => *section_id,
                    )*
                }
            }

            pub fn set_section_id(&mut self, section_id: i32) {
                match self {
                    $(
                        $name::$variant { section_id: ref mut s, .. } => *s = section_id,
                    )*
                }
            }

            #[must_use]
            pub const fn get_open_ui(&self) -> UIType {
                match self {
                    $(
                        $name::$variant { open_ui, .. } => *open_ui,
                    )*
                }
            }

            pub fn set_open_ui(&mut self, open_ui: UIType) {
                match self {
                    $(
                        $name::$variant { open_ui: ref mut o, .. } => *o = open_ui,
                    )*
                }
            }

            #[must_use]
            pub const fn get_to_be_destroyed(&self) -> bool {
                match self {
                    $(
                        $name::$variant { to_be_destroyed, .. } => *to_be_destroyed,
                    )*
                }
            }

            pub fn set_to_be_destroyed(&mut self, to_be_destroyed: bool) {
                match self {
                    $(
                        $name::$variant { to_be_destroyed: ref mut t, .. } => *t = to_be_destroyed,
                    )*
                }
            }

            #[must_use]
            pub const fn get_camera_x(&self) -> u32 {
                match self {
                    $(
                        $name::$variant { camera_x, .. } => *camera_x,
                    )*
                }
            }

            pub fn set_camera_x(&mut self, camera_x: u32) {
                match self {
                    $(
                        $name::$variant { camera_x: ref mut c, .. } => *c = camera_x,
                    )*
                }
            }

            #[must_use]
            pub const fn get_camera_y(&self) -> u32 {
                match self {
                    $(
                        $name::$variant { camera_y, .. } => *camera_y,
                    )*
                }
            }

            pub fn set_camera_y(&mut self, camera_y: u32) {
                match self {
                    $(
                        $name::$variant { camera_y: ref mut c, .. } => *c = camera_y,
                    )*
                }
            }
        }
    };
}

macro_rules! polymorphic_item_info {
    (enum $name:ident { $($variant:ident { $($field:ident: $ty:ty),* $(,)? } = $tag:expr,)* }) => {
        #[derive(OctData, Clone, Debug)]
        #[repr(u16)]
        #[base = 5]
        pub enum $name {
            $(
                $variant {
                    uid: u64,
                    id: i32,
                    count: i32,
                    package: u16,
                    first_get_time: u64,
                    $($field: $ty),*
                } = $tag,
            )*
        }

        impl $name {
            #[must_use]
            pub const fn get_uid(&self) -> u64 {
                match self {
                    $(
                        $name::$variant { uid, .. } => *uid,
                    )*
                }
            }

            pub fn set_uid(&mut self, uid: u64) {
                match self {
                    $(
                        $name::$variant { uid: ref mut u, .. } => *u = uid,
                    )*
                }
            }

            #[must_use]
            pub const fn get_id(&self) -> i32 {
                match self {
                    $(
                        $name::$variant { id, .. } => *id,
                    )*
                }
            }

            pub fn set_id(&mut self, id: i32) {
                match self {
                    $(
                        $name::$variant { id: ref mut i, .. } => *i = id,
                    )*
                }
            }

            #[must_use]
            pub const fn get_count(&self) -> i32 {
                match self {
                    $(
                        $name::$variant { count, .. } => *count,
                    )*
                }
            }

            pub fn set_count(&mut self, count: i32) {
                match self {
                    $(
                        $name::$variant { count: ref mut c, .. } => *c = count,
                    )*
                }
            }

            #[must_use]
            pub const fn get_package(&self) -> u16 {
                match self {
                    $(
                        $name::$variant { package, .. } => *package,
                    )*
                }
            }

            pub fn set_package(&mut self, package: u16) {
                match self {
                    $(
                        $name::$variant { package: ref mut p, .. } => *p = package,
                    )*
                }
            }

            #[must_use]
            pub const fn get_first_get_time(&self) -> u64 {
                match self {
                    $(
                        $name::$variant { first_get_time, .. } => *first_get_time,
                    )*
                }
            }

            pub fn set_first_get_time(&mut self, first_get_time: u64) {
                match self {
                    $(
                        $name::$variant { first_get_time: ref mut f, .. } => *f = first_get_time,
                    )*
                }
            }
        }
    };
}

macro_rules! polymorphic_dungeon_table_ext {
    (enum $name:ident { $($variant:ident { $($field:ident: $ty:ty),* $(,)? } = $tag:expr,)* }) => {
        #[derive(OctData, Clone, Debug)]
        #[repr(u16)]
        #[base = 0]
        pub enum $name {
            $(
                $variant {
                    $($field: $ty),*
                } = $tag,
            )*
        }
    };
}

macro_rules! polymorphic_scene_table_ext {
    (enum $name:ident { $($variant:ident { $($field:ident: $ty:ty),* $(,)? } = $tag:expr,)* }) => {
        #[derive(OctData, Clone, Debug)]
        #[repr(u16)]
        #[base = 1]
        pub enum $name {
            $(
                $variant {
                    event_graphs_info: EventGraphsInfo,
                    $($field: $ty),*
                } = $tag,
            )*
        }

        impl $name {
            #[must_use]
            pub const fn get_event_graphs_info(&self) -> &EventGraphsInfo {
                match self {
                    $(
                        $name::$variant { event_graphs_info, .. } => event_graphs_info,
                    )*
                }
            }

            pub fn set_event_graphs_info(&mut self, event_graphs_info: EventGraphsInfo) {
                match self {
                    $(
                        $name::$variant { event_graphs_info: ref mut e, .. } => *e = event_graphs_info,
                    )*
                }
            }
        }
    };
}

macro_rules! polymorphic_section_info_ext {
    (enum $name:ident { $($variant:ident { $($field:ident: $ty:ty),* $(,)? } = $tag:expr,)* }) => {
        #[derive(OctData, Clone, Debug)]
        #[repr(u16)]
        #[base = 1]
        pub enum $name {
            $(
                $variant {
                    destroy_npc_when_no_player: PropertyHashSet<u64>,
                    $($field: $ty),*
                } = $tag,
            )*
        }

        impl $name {
            #[must_use]
            pub const fn get_destroy_npc_when_no_player(&self) -> &PropertyHashSet<u64> {
                match self {
                    $(
                        $name::$variant { destroy_npc_when_no_player, .. } => destroy_npc_when_no_player,
                    )*
                }
            }

            pub fn set_destroy_npc_when_no_player(&mut self, destroy_npc_when_no_player: PropertyHashSet<u64>) {
                match self {
                    $(
                        $name::$variant { destroy_npc_when_no_player: ref mut d, .. } => *d = destroy_npc_when_no_player,
                    )*
                }
            }
        }
    };
}

macro_rules! polymorphic_action_info {
    (enum $name:ident { $($variant:ident { $($field:ident: $ty:ty),* $(,)? } = $tag:expr,)* }) => {
        #[derive(OctData, Clone, Debug)]
        #[repr(u16)]
        #[base = 0]
        pub enum $name {
            $(
                $variant {
                    $($field: $ty),*
                } = $tag,
            )*
            #[polymorphic_none]
            None {} = 0xFFFF, // weird detail, polymorphism can be empty.
        }
    };
}

macro_rules! polymorphic_event_graph_info {
    (enum $name:ident { $($variant:ident { $($field:ident: $ty:ty),* $(,)? } = $tag:expr,)* }) => {
        #[derive(OctData, Clone, Debug)]
        #[repr(u16)]
        #[base = 6]
        pub enum $name {
            $(
                $variant {
                    config_id: i32,
                    events_info: PropertyHashMap<i32, EventInfo>,
                    specials: PropertyHashMap<String, u64>,
                    is_new: bool,
                    finished: bool,
                    list_specials: PropertyHashMap<String, Vec<u64>>,
                    $($field: $ty),*
                } = $tag,
            )*
        }

        impl $name {
            #[must_use]
            pub const fn get_config_id(&self) -> i32 {
                match self {
                    $(
                        $name::$variant { config_id, .. } => *config_id,
                    )*
                }
            }

            pub fn set_config_id(&mut self, config_id: i32) {
                match self {
                    $(
                        $name::$variant { config_id: ref mut c, .. } => *c = config_id,
                    )*
                }
            }

            #[must_use]
            pub const fn get_events_info(&self) -> &PropertyHashMap<i32, EventInfo> {
                match self {
                    $(
                        $name::$variant { events_info, .. } => events_info,
                    )*
                }
            }

            pub fn set_events_info(&mut self, events_info: PropertyHashMap<i32, EventInfo>) {
                match self {
                    $(
                        $name::$variant { events_info: ref mut e, .. } => *e = events_info,
                    )*
                }
            }

            #[must_use]
            pub const fn get_specials(&self) -> &PropertyHashMap<String, u64> {
                match self {
                    $(
                        $name::$variant { specials, .. } => specials,
                    )*
                }
            }

            pub fn set_specials(&mut self, specials: PropertyHashMap<String, u64>) {
                match self {
                    $(
                        $name::$variant { specials: ref mut s, .. } => *s = specials,
                    )*
                }
            }

            #[must_use]
            pub const fn get_is_new(&self) -> bool {
                match self {
                    $(
                        $name::$variant { is_new, .. } => *is_new,
                    )*
                }
            }

            pub fn set_is_new(&mut self, is_new: bool) {
                match self {
                    $(
                        $name::$variant { is_new: ref mut i, .. } => *i = is_new,
                    )*
                }
            }

            #[must_use]
            pub const fn get_finished(&self) -> bool {
                match self {
                    $(
                        $name::$variant { finished, .. } => *finished,
                    )*
                }
            }

            pub fn set_finished(&mut self, finished: bool) {
                match self {
                    $(
                        $name::$variant { finished: ref mut f, .. } => *f = finished,
                    )*
                }
            }

            #[must_use]
            pub const fn get_list_specials(&self) -> &PropertyHashMap<String, Vec<u64>> {
                match self {
                    $(
                        $name::$variant { list_specials, .. } => list_specials,
                    )*
                }
            }

            pub fn set_list_specials(&mut self, list_specials: PropertyHashMap<String, Vec<u64>>) {
                match self {
                    $(
                        $name::$variant { list_specials: ref mut l, .. } => *l = list_specials,
                    )*
                }
            }
        }
    };
}

macro_rules! polymorphic_quest_info {
    (enum $name:ident { $($variant:ident { $($field:ident: $ty:ty),* $(,)? } = $tag:expr,)* }) => {
        #[derive(OctData, Clone, Debug)]
        #[repr(u16)]
        #[base = 9]
        pub enum $name {
            $(
                $variant {
                    id: i32,
                    finished_count: i32,
                    collection_uid: u64,
                    progress: u16,
                    parent_quest_id: i32,
                    state: QuestState,
                    finish_condition_progress: PropertyHashMap<i32, i32>,
                    progress_time: u32,
                    sort_id: u64,
                    $($field: $ty),*
                } = $tag,
            )*
        }

        impl $name {
            #[must_use]
            pub const fn get_id(&self) -> i32 {
                match self {
                    $(
                        $name::$variant { id, .. } => *id,
                    )*
                }
            }

            pub fn set_id(&mut self, id: i32) {
                match self {
                    $(
                        $name::$variant { id: ref mut c, .. } => *c = id,
                    )*
                }
            }

            #[must_use]
            pub const fn get_collection_uid(&self) -> u64 {
                match self {
                    $(
                        $name::$variant { collection_uid, .. } => *collection_uid,
                    )*
                }
            }

            pub fn set_collection_uid(&mut self, collection_uid: u64) {
                match self {
                    $(
                        $name::$variant { collection_uid: ref mut c, .. } => *c = collection_uid,
                    )*
                }
            }

            pub fn set_state(&mut self, state: QuestState) {
                match self {
                    $(
                        $name::$variant { state: ref mut c, .. } => *c = state,
                    )*
                }
            }

            pub fn set_progress(&mut self, progress: u16) {
                match self {
                    $(
                        $name::$variant { progress: ref mut c, .. } => *c = progress,
                    )*
                }
            }

            pub fn set_finished_count(&mut self, finished_count: i32) {
                match self {
                    $(
                        $name::$variant { finished_count: ref mut c, .. } => *c = finished_count,
                    )*
                }
            }
        }
    };
}

polymorphic_scene_unit_protocol_info! {
    enum SceneUnitProtocolInfo {
        NpcProtocolInfo {
            id: i32,
            quest_id: i32,
            interacts_info: PropertyHashMap<i32, InteractInfo>,
        } = 0,
    }
}

polymorphic_scene_info! {
    enum SceneInfo {
        Fight {
            perform_show_progress: PropertyHashMap<ACTPerformShowMoment, u8>,
            end_hollow: bool,
            random_seed: i32,
        } = 3,
        Fresh {} = 4,
        Hall {
            // main_city_time_info: MainCityTimeInfo,
        } = 1,
        Hollow {
            event_variables: PropertyHashMap<String, i32>,
            buddy: BuddyUnitInfo,
            stress_punish_ability_random_pool: Vec<String>,
            finished: bool,
            event_weight_factor: PropertyHashMap<i32, i32>,
            shop_modification: HollowShopModification,
            last_challenge_stat: PropertyHashMap<i32, u8>,
            cur_challenge: PropertyHashSet<i32>,
            hollow_system_switch: PropertyHashMap<HollowSystemType, bool>,
            sections_info: PropertyHashMap<i32, PlayerHollowSectionInfo>,
            executing_event: bool,
            event_id: i32,
            hollow_event_graph_uid: u64,
            on_battle_success: String,
            on_battle_failure: String,
            battle_finished: bool,
            battle_success: bool,
            battle_scene_uid: u64,
            //event_graphs_info: PropertyHashMap<u64, HollowEventGraphInfo>,
            scene_global_events: PropertyHashMap<i32, u64>,
            prepare_section: PrepareSection,
            abilities_info: AbilitiesInfo,
            blackout: bool,
            hollow_system_ui_state: PropertyHashMap<HollowSystemType, HollowSystemUIState>,
        } = 2,
    }
}

polymorphic_item_info! {
    enum ItemInfo {
        Arcana {
            affix_list: Vec<i32>,
            dress_index: u8,
        } = 33,
        Avatar {
            star: u8,
            exp: u32,
            level: u8,
            rank: u8,
            unlocked_talent_num: u8,
            skills: PropertyHashMap<u8, u8>,
            is_custom_by_dungeon: bool,
            robot_id: i32,
        } = 3,
        AvatarLevelUpMaterial { } = 12,
        AvatarPiece { } = 4,
        Bless {
            remain_time: i32,
            get_time: u64,
            ban_character: Vec<i32>,
            specials: PropertyHashMap<String, i32>,
            slot: u8,
            is_super_curse: bool,
        } = 32,
        Buddy { } = 8,
        Consumable { } = 10,
        Currency { } = 1,
        Equip {
            avatar_uid: u64,
            avatar_dressed_index: u8,
            rand_properties: Vec<PropertyKeyValue>,
            star: u8,
            exp: u32,
            leve: u8,
            lock: u8,
            base_rand_properties: Vec<PropertyKeyValue>,
            rand_properties_lv: Vec<i32>,
        } = 7,
        EquipLevelUpMaterial { } = 14,
        Gift { } = 51,
        HollowItem { } = 15,
        OptionalGift { } = 52,
        Resource { } = 2,
        TarotCard {
            is_mute: bool,
            specials: PropertyHashMap<String, i32>,
        } = 31,
        Useable { } = 11,
        Weapon {
            avatar_uid: u64,
            star: u8,
            exp: u32,
            level: u8,
            lock: u8,
            refine_level: u8,
        } = 5,
        WeaponLevelUpMaterial { } = 13,
    }
}

polymorphic_dungeon_table_ext! {
    enum DungeonTableExt {
        Hall {} = 1,
        Hollow {
            avatars: PropertyHashSet<HollowDungeonAvatarInfo>,
            scene_properties_uid: u64,
            buddy: HollowDungeonBuddyInfo,
        } = 2,
    }
}

polymorphic_section_info_ext! {
    enum SectionInfoExt {
        Hall {} = 1,
        Hollow {
            hollow_level_info: HollowLevelInfo,
            hollow_grid_map_info: HollowGridMapInfo,
        } = 0,
    }
}

polymorphic_scene_table_ext! {
    enum SceneTableExt {
        Fight {} = 3,
        Fresh {} = 4,
        Hall {} = 1,
        Hollow {
            grid_random_seed: i32,
            alter_section_id: i32,
        } = 2,
    }
}

polymorphic_action_info! {
    enum ActionInfo {
        ServerChoices {
            choices: Vec<ChoiceInfo>,
            finished: bool,
        } = 52,
        DropHollowItem {
            drop_item: i32,
        } = 162,
        FinishBlackout {
            finished: bool,
            show_tips: bool,
        } = 133,
        Loop {
            loop_times: u16,
        } = 141,
        Perform {
            step: u8,
            r#return: PropertyHashMap<String, i32>,
        } = 23,
        PrepareNextHollow {
            section_id: i32,
            finished: bool,
            show_other: bool,
            main_map: HollowGridMapProtocolInfo,
        } = 130,
        ActionRandomChallenge {
            choices: Vec<i32>,
            choice_result: i32,
            finished: bool,
        } = 109,
        RemoveCurse {
            curse_can_remove: Vec<u64>,
            to_remove_num: u8,
            choosed: bool,
        } = 105,
        SetHollowSystemState {
            finished: bool,
        } = 134,
        Shop {
            shop_info: PropertyHashMap<HollowShopType, ConfigShopInfo>,
            finished: bool,
        } = 62,
        SlotMachine {
            indexes: Vec<i32>,
            index: i32,
            finished: bool,
        } = 131,
        TriggerBattle {
            next_action_id: i32,
            finished: bool,
        } = 56,
    }
}

polymorphic_event_graph_info! {
    enum EventGraphInfo {
        Hollow {
            fired_count: u8,
            hollow_event_template_id: i32,
            uid: u64,
            is_created_by_gm: bool,
        } = 3,
        NPC {
            sequence_of_group: u16,
            section_list_events: PropertyHashMap<String, EventListenerInfo>,
            interact_info: InteractInfo,
            hide: bool,
        } = 2,
        Section { } = 1,
    }
}

polymorphic_quest_info! {
    enum QuestInfo {
        ArchiveBattle {
            statistics: PropertyHashMap<QuestStatisticsType, u64>,
            dungeon_uid: u64,
            star: u8,
        } = 7,
        ArchiveFile { } = 1,
        Challenge { } = 6,
        DungeonInner { } = 2,
        Hollow {
            statistics: PropertyHashMap<QuestStatisticsType, u64>,
            dungeon_uid: u64,
            statistics_ext: PropertyDoubleKeyHashMap<QuestStatisticsType, i32, i32>,
            acquired_hollow_challenge_reward: i32,
        } = 3,
        Knowledge { } = 8,
        MainCity {
            bound_npc_and_interact: PropertyHashMap<u64, BoundNPCAndInteractInfo>,
        } = 5,
        Manual { } = 4,
    }
}
