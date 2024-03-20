use super::*;

// :skull:
macro_rules! ret {
    (struct $name:ident $(< $lt:lifetime >)? {
    $(
        $(#[$attr:meta])*
        $field:ident: $ty:ty,
    )*
    }) => {
        #[derive(OctData)]
        pub struct $name $(< $lt >)? {
            pub error_code: ErrorCode,
            pub error_code_params: Vec<String>,
            $(
                $(#[$attr])*
                pub $field: $ty,
            )*
        }

        impl $(< $lt >)? $name $(< $lt >)? {
            #[must_use]
            pub const fn new($($field: $ty,)*) -> Self {
                Self {
                    error_code: ErrorCode::Success,
                    error_code_params: Vec::new(),
                    $($field,)*
                }
            }

            #[must_use]
            pub fn error(error_code: ErrorCode, error_code_params: Vec<String>) -> Self {
                Self {
                    error_code,
                    error_code_params,
                    $($field: Default::default(),)*
                }
            }
        }

        impl $(< $lt >)? Default for $name $(< $lt >)? {
            fn default() -> Self {
                Self {
                    error_code: ErrorCode::Success,
                    error_code_params: Vec::new(),
                    $($field: Default::default(),)*
                }
            }
        }
    };
    ($(struct $name:ident $(< $lt:lifetime >)? { $($field:tt)* })+) => {
        $(ret!(struct $name $(< $lt >)? { $($field)* });)+
    };
}

#[derive(OctData, Debug)]
pub struct RpcLoginArg {
    pub account_name: String,
    pub token: String,
    pub client_protocol_sign: String,
    pub config_sign: String,
}

#[derive(OctData, Clone, Debug)]
pub struct PtcEnterSceneArg {
    pub player_uid: u64,
    pub scene_uid: u64,
    pub ext: SceneTableExt,
    pub entered_times: u16,
    pub section_id: i32,
    pub transform: Transform,
    pub open_ui: UIType,
    pub condition_config_ids: Vec<i32>,
    pub timestamp: u64,
    pub camera_x: u32,
    pub camera_y: u32,
}

#[derive(OctData, Clone, Debug)]
pub struct RpcEnterWorldArg {}

#[derive(OctData, Clone, Debug)]
pub struct RpcGetPlayerMailsArg {}

#[derive(OctData, Clone, Debug)]
pub struct PtcUnlockArg {
    pub unlock_id: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct PtcGetServerTimestampArg {}

#[derive(OctData, Clone, Debug)]
pub struct RpcAdvanceBeginnerProcedureArg {
    pub player_uid: u64,
    pub procedure_id: i32,
    pub params: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct RpcPerformTriggerArg {
    pub perform_id: i32,
    pub perform_type: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct RpcPerformEndArg {
    pub perform_id: i32,
    pub perform_type: i32,
    pub perform_uid: String,
}

#[derive(OctData, Clone, Debug)]
pub struct RpcModNickNameArg {
    pub nick_name: String,
    pub avatar_id: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct RpcFinishACTPerformShowArg {
    pub moment: ACTPerformShowMoment,
    pub step: u8,
}

#[derive(OctData, Clone, Debug)]
pub struct RpcKeepAliveArg {}

#[derive(OctData, Clone, Debug)]
pub struct RpcPerformJumpArg {
    pub perform_id: i32,
    pub perform_type: i32,
    pub perform_uid: String,
}

#[derive(OctData, Clone, Debug)]
pub struct RpcBeginnerbattleBeginArg {
    pub battle_id: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct RpcBattleReportArg {
    pub battle_reports: Vec<BattleReport>,
}

#[derive(OctData, Clone, Debug)]
pub struct RpcBeginnerbattleEndArg {
    pub battle_id: i32,
    pub battle_uid: String,
    pub battle_statistics: LogBattleStatistics,
}

#[derive(OctData, Clone, Debug)]
pub struct RpcLeaveCurDungeonArg {
    pub player_uid: u64,
    pub dungeon_uid: u64,
}

#[derive(OctData, Clone, Debug)]
pub struct RpcSavePosInMainCityArg {
    pub position: Vector3f,
    pub rotation: Vector3f,
}

#[derive(OctData, Clone, Debug)]
pub struct RpcCloseLevelChgTipsArg {}

#[derive(OctData, Clone, Debug)]
pub struct PtcPlayerInfoChangedArg {
    pub player_uid: u64,
    #[property_blob]
    pub player_info: PlayerInfo,
}

#[derive(OctData, Clone, Debug)]
pub struct PtcPlayerOperationArg {
    pub system: System,
    pub operator: Operator,
    pub param: i32,
}

#[derive(OctData, Debug)]
pub struct PtcScenePropertyChangedArg {
    pub player_uid: u64,
    pub is_partial: bool,
    pub changed_properties: PropertyHashMap<u16, i32>,
}

#[derive(OctData, Debug)]
pub struct PtcPropertyChangedArg {
    pub scene_unit_uid: u64,
    pub is_partial: bool,
    pub changed_properties: PropertyHashMap<u16, i32>,
}

#[derive(OctData, Debug)]
pub struct PtcSyncSceneUnitArg {
    pub scene_uid: u64,
    pub section_id: i32,
    pub is_partial: bool,
    pub removed_scene_units: Vec<u64>,
    pub scene_units: Vec<SceneUnitProtocolInfo>,
}

#[derive(OctData, Debug)]
pub struct PtcEnterSectionArg {
    pub section_id: i32,
}

#[derive(OctData, Debug)]
pub struct RpcRunEventGraphArg {
    pub owner_type: EventGraphOwnerType,
    pub owner_uid: u64,
    pub event_graph_id: i32,
    pub event_id: i32,
    pub move_path: Vec<i32>,
}

#[derive(OctData, Debug)]
pub struct RpcInteractWithUnitArg {
    pub unit_uid: u64,
    pub unit_type: InteractTarget,
    pub event_graph_id: i32,
    pub interaction: u16,
}

#[derive(OctData, Debug)]
pub struct PtcSyncEventInfoArg {
    pub owner_type: EventGraphOwnerType,
    pub owner_uid: u64,
    pub updated_events: PropertyDoubleKeyHashMap<i32, i32, EventInfo>,
}

#[derive(OctData, Debug)]
pub struct RpcCheckYorozuyaInfoRefreshArg {}

#[derive(OctData, Debug)]
pub struct PtcHollowQuestUnlockedByMainCityQuest {
    pub quest_id: i32,
}

#[derive(OctData, Debug)]
pub struct RpcStartHollowQuestArg {
    pub hollow_quest_id: i32,
    pub buddy: u64,
    pub initiative_item: i32,
    pub avatar_map: PropertyHashMap<i8, u64>,
    pub is_story: bool,
}

#[derive(OctData, Debug)]
pub struct PtcSyncHollowGridMapsArg {
    pub player_uid: u64,
    pub scene_uid: u64,
    pub hollow_level: i32,
    pub main_map: HollowGridMapProtocolInfo,
    pub time_period: TimePeriodType,
    pub weather: WeatherType,
}

#[derive(OctData, Debug)]
pub struct PtcPositionInHollowChangedArg {
    pub player_uid: u64,
    pub hollow_level: i32,
    pub position: u16,
}

#[derive(OctData, Debug)]
pub struct PtcSyncHollowEventInfoArg {
    pub event_graph_uid: u64,
    pub hollow_event_template_id: i32,
    pub event_graph_id: i32,
    pub updated_event: EventInfo,
    pub specials: PropertyHashMap<String, i32>,
}

#[derive(OctData, Debug)]
pub struct RpcRunHollowEventGraphArg {
    pub event_graph_uid: u64,
    pub event_id: i32,
    pub move_path: Vec<i32>,
}

#[derive(OctData, Debug)]
pub struct PtcHollowGridArg {
    pub player_uid: u64,
    pub is_partial: bool,
    pub scene_uid: u64,
    pub hollow_level: i32,
    pub grids: HashMap<u16, HollowGridProtocolInfo>,
}

#[derive(OctData, Debug)]
pub struct RpcHollowMoveArg {
    pub player_uid: u64,
    pub scene_uid: u64,
    pub hollow_level: i32,
    pub positions: Vec<u16>,
}

#[derive(OctData, Debug)]
pub struct RpcEndBattleArg {
    pub player_uid: u64,
    pub fight_ranking: FightRanking,
    pub success: bool,
    pub avatar_properties: PropertyHashMap<u64, HashMap<u16, i32>>,
    pub killed_enemy_count: u16,
    pub condition_statistics: HashMap<i32, i32>,
    pub star: u8,
    pub challenge_stat: HashMap<i32, u8>,
    pub fight_drop_infos: Vec<FightDropInfo>,
    pub challenge_result_info: PropertyHashMap<i32, ChallengeResultInfo>,
    pub battle_statistics: LogBattleStatistics,
}

#[derive(OctData, Debug)]
pub struct RpcFinishEventGraphPerformShowArg {
    pub owner_type: EventGraphOwnerType,
    pub owner_uid: u64,
    pub event_graph_id: i32,
    pub event_id: i32,
    pub step: u8,
    pub return_map: HashMap<String, i32>,
}

#[derive(OctData, Debug)]
pub struct RpcDelNewMapArg {
    pub map_type: UnlockIDType,
    pub ids: PropertyHashSet<i32>,
}

#[derive(OctData, Debug)]
pub struct PtcDungeonQuestFinishedArg {
    pub player_uid: u64,
    pub quest_id: i32,
    pub success: bool,
    pub reward_items: PropertyHashMap<u64, ItemIDCount>,
    pub statistics: PropertyHashMap<QuestStatisticsType, u64>,
}

ret! {
    struct RpcLoginRet {
        account_info: PropertyBlob,
    }

    struct RpcEnterWorldRet {
        player_info: PropertyBlob,
    }

    struct RpcGetPlayerMailsRet {
        mail_count: u32, // Actually List<CPlayerMailInfo>, TODO!
    }

    struct PtcGetServerTimestampRet {
        timestamp: u64,
        base_utc_offset_milliseconds: i64,
    }

    struct RpcAdvanceBeginnerProcedureRet {
        next_procedure_id: i32,
    }

    struct RpcPerformTriggerRet {
        perform_uid: String,
    }

    struct RpcPerformEndRet {
    }

    struct RpcModNickNameRet {
    }

    struct RpcFinishACTPerformShowRet {
    }

    struct RpcKeepAliveRet {
    }

    struct RpcPerformJumpRet {
    }

    struct RpcBeginnerbattleBeginRet {
        battle_uid: String,
    }

    struct RpcBattleReportRet {
        need_index: i32,
    }

    struct RpcBeginnerbattleEndRet {
    }

    struct RpcLeaveCurDungeonRet {
    }

    struct RpcSavePosInMainCityRet {
    }

    struct RpcCloseLevelChgTipsRet {
    }

    struct PtcPlayerOperationRet {
    }

    struct RpcRunEventGraphRet {
    }

    struct RpcInteractWithUnitRet {
    }

    struct RpcCheckYorozuyaInfoRefreshRet {
    }

    struct RpcStartHollowQuestRet {
    }

    struct RpcRunHollowEventGraphRet {
    }

    struct RpcHollowMoveRet {
        hollow_level: i32,
        position: u16,
    }

    struct RpcEndBattleRet {
        hollow_event_id: i32,
        reward_items_classify: HashMap<BattleRewardType, HashMap<u64, ItemIDCount>>,
    }

    struct RpcFinishEventGraphPerformShowRet {
    }

    struct RpcDelNewMapRet {
    }
}
