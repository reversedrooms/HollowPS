use super::*;

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ErrorCode {
    Fail = -1,
    Success = 0,
}

#[derive(OctData, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i16)]
pub enum HollowQuestType {
    Common = 0,
    MainQuest = 1,
    SideQuest = 2,
    Urgent = 3,
    UrgentSupplement = 4,
    Challenge = 5,
    ChallengeChaos = 6,
    AvatarSide = 7,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FairyState {
    Unlock = 0,
    Close = 1,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum FightRanking {
    None = 0,
    D = 1,
    C = 2,
    B = 3,
    A = 4,
    S = 5,
}

#[derive(OctData, Hash, Clone, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum BattleRewardType {
    Client = 1,
    BattleEvt = 2,
    Ext = 3,
    Fight = 4,
    Challenge = 5,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum MailState {
    New = 0,
    Old = 1,
    Read = 2,
    Awarded = 3,
    Removed = 4,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum HollowBattleEventType {
    Default = 0,
    Normal = 1,
    Elite = 2,
    Boss = 3,
    LevelEnd = 4,
    LevelFin = 5,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i16)]
pub enum QuestType {
    ArchiveFile = 1,
    DungeonInner = 2,
    Hollow = 3,
    Manual = 4,
    MainCity = 5,
    HollowChallenge = 6,
    ArchiveBattle = 7,
    Knowledge = 8,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum EventState {
    Initing = 0,
    Running = 1,
    Pause = 2,
    WaitingMsg = 3,
    WaitingClient = 4,
    Finished = 5,
    Error = 6,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum ActionState {
    Init = 0,
    Running = 1,
    Finished = 2,
    Error = 3,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DungeonContentDropPoolType {
    Card = 0,
    BaneCard = 1,
    Arcana = 2,
    Blessing = 3,
    Curse = 4,
    Reward = 5,
    HollowItem = 6,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum ReportType {
    Fairy = 0,
    Dialog = 1,
    Task = 2,
    DialogInFairy = 3,
}

#[derive(OctData, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum UIType {
    Default = 0,
    None = 1,
    HollowQuest = 2,
    Archive = 3,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i16)]
pub enum ACTPerformShowMoment {
    Begin = 0,
    End = 1,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i16)]
pub enum HollowSystemType {
    Card = 1,
    Menu = 2,
    Curse = 3,
    Bag = 4,
    HollowItem = 5,
    HollowResultPage = 6,
    CurseInfo = 7,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum HollowSystemUIState {
    Normal = 0,
    Close = 1,
    Brighten = 2,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i16)]
pub enum HollowShopType {
    All = 0,
    Item = 1,
    Card = 2,
    Curse = 3,
    HollowItem = 4,
    Discount = 5,
    Gachashop = 6,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i16)]
pub enum TimePeriodType {
    Random = 0,
    Morning = 1,
    Evening = 2,
    Night = 3,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i16)]
pub enum WeatherType {
    None = -1,
    Random = 0,
    SunShine = 1,
    Fog = 2,
    Cloudy = 3,
    Rain = 4,
    Thunder = 5,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i16)]
pub enum PropertyType {
    Hp = 1,
    Armor = 2,
    Shield = 3,
    Stun = 4,
    Sp = 5,
    Usp = 6,
    Dead = 99,
    HpMax = 111,
    ArmorMax = 112,
    ShieldMax = 113,
    StunMax = 114,
    SpMax = 115,
    UspMax = 116,
    Atk = 121,
    BreakStun = 122,
    Def = 131,
    Crit = 201,
    CritRes = 202,
    CritDmg = 211,
    CritDmgRes = 212,
    Pen = 231,
    PenValue = 232,
    Endurance = 301,
    SpRecover = 305,
    HpHealRatio = 306,
    AddedDamageRatio = 307,
    HpMaxBattle = 1111,
    ArmorMaxBattle = 1112,
    ShieldMaxBattle = 1113,
    StunMaxBattle = 1114,
    SpBattle = 1115,
    // UspBattle = 1115,
    AtkBattle = 1121,
    BreakStunBattle = 1122,
    DefBattle = 1131,
    CritBattle = 1201,
    CritResBattle = 1202,
    CritDmgBattle = 1211,
    CritDmgResBattle = 1212,
    PenRatioBattle = 1231,
    PenDeltaBattle = 1232,
    EnduranceBattle = 1301,
    SpRecoverBattle = 1305,
    HpHealRatioBattle = 1306,
    AddedDamageRatioBattle = 1307,
    HpMaxBase = 11101,
    ArmorMaxBase = 11201,
    ShieldMaxBase = 11301,
    AtkBase = 12101,
    DefBase = 13101,
    CritBase = 20101,
    CritResBase = 20201,
    CritDmgBase = 21101,
    CritDmgResBase = 21201,
    PenBase = 23101,
    PenValueBase = 23201,
    BreakStunBase = 12201,
    StunMaxBase = 11401,
    SpMaxBase = 11501,
    EnduranceBase = 30101,
    UspMaxBase = 11601,
    SpRecoverBase = 30501,
    HpHealRatio1 = 30601,
    AddedDamageRatio1 = 30701,
    HpMaxRatio = 11102,
    ArmorMaxRatio = 11202,
    ShieldMaxRatio = 11302,
    AtkRatio = 12102,
    DefRatio = 13102,
    BreakStunRatio = 12202,
    StunMaxRatio = 11402,
    EnduranceRatio = 30102,
    SpRecoverRatio = 30502,
    HpMaxDelta = 11103,
    ArmorMaxDelta = 11203,
    ShieldMaxDelta = 11303,
    AtkDelta = 12103,
    DefDelta = 13103,
    BreakStunDelta = 12203,
    StunMaxDelta = 11403,
    SpMaxDelta = 11503,
    CritDelta = 20103,
    CritResDelta = 20203,
    CritDmgDelta = 21103,
    CritDmgResDelta = 21203,
    UspMaxDelta = 11603,
    PenDelta = 23103,
    PenValueDelta = 23203,
    EnduranceDelta = 30103,
    SpRecoverDelta = 30503,
    HpHealRatio3 = 30603,
    AddedDamageRatio3 = 30703,
    HpMaxRatioRL = 11104,
    ArmorMaxRatioRL = 11204,
    ShieldMaxRatioRL = 11304,
    AtkRatioRL = 12104,
    DefRatioRL = 13104,
    HpMaxDeltaRL = 11105,
    ArmorMaxDeltaRL = 11205,
    ShieldMaxDeltaRL = 11305,
    AtkDeltaRL = 12105,
    DefDeltaRL = 13105,
    CritRL = 20105,
    CritResRL = 20205,
    CritDmgRL = 21105,
    CritDmgResRL = 21205,
    PenRatioRL = 23105,
    PenDeltaRL = 23205,
    BreakStunRatioRL = 12204,
    BreakStunDeltaRL = 12205,
    StunMaxRatioRL = 11404,
    // StunMaxDeltaRL = 11404,
    SpMaxDeltaRL = 11505,
    UspMaxDeltaRL = 11605,
    EnduranceRatioRL = 30104,
    EnduranceDeltaRL = 30105,
    SpRecoverRatioRL = 30504,
    SpRecoverDeltaRL = 30505,
    HpHealRatioRL = 30605,
    AddedDamageRatioRL = 30705,
    MapHpreserveMaxhp = 10320,
    MapHpreserveCurhp = 10330,
    MapHpreserveAbsolute = 10340,
    ActorMaxCurHP = 10350,
    EnumCount = 10351,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i16)]
pub enum ScenePropertyType {
    Stamina = 1001,
    StaminaMax = 1002,
    StaminaRatio = 1003,
    StaminaDelta = 1004,
    GoldRatio = 1005,
    GoldDelta = 1006,
    CardRWeight = 1007,
    CardRWeightRatio = 1008,
    CardSRWeight = 1009,
    CardSRWeightRatio = 1010,
    CardSSRWeight = 1011,
    CardSSRWeightRatio = 1012,
    Mobility = 1013,
    BuffTurn = 1014,
    ForbiddenStamina = 1015,
    ForbiddenGold = 1016,
    OptionNum = 1017,
    ShopPrice = 1018,
    StaminaIncrease = 1019,
    StaminaOverLevel = 1020,
    DropRate = 1021,
    BanCharacter1 = 1022,
    BanCharacter2 = 1023,
    BanCharacter3 = 1024,
    PlayerView = 1025,
    ActorAddedDamageRatio = 1030,
    ActorDamageTakeRatio = 1031,
    MapHpreserveMaxhp = 1032,
    MapHpreserveCurhp = 1033,
    MapHpreserveAbsolute = 1034,
    ActorMaxCurHP = 1035,
    ShopPriceDelta = 1036,
    ShopPriceOverwriteCard = 1037,
    ShopPriceOverwriteItem = 1038,
    CardOptionHideNum = 1039,
    CardOptionForbidNum = 1040,
    HealingRatio = 1041,
    DinyRatio = 1042,
    Weather = 1043,
    TimePeriod = 1044,
    ShopPriceOverwriteCurse = 1045,
    ShopPriceOverwriteHollowItem = 1046,
    ShopPriceOverwriteDiscount = 1047,
    ShopPriceOverwriteGachashop = 1048,
}

macro_rules! flag {
    ($repr:ty, $(#[$attr:meta])* $name:ident { $($flag:ident = $value:expr,)* }) => {
        $(#[$attr])*
        #[repr($repr)]
        pub enum $name {
            $($flag = $value,)*
        }

        impl From<$name> for $repr {
            fn from(flag: $name) -> $repr {
                flag as $repr
            }
        }

        impl From<$repr> for $name {
            fn from(flag: $repr) -> $name {
                match flag {
                    $($value => $name::$flag,)*
                    _ => panic!("invalid flag value: {}", flag),
                }
            }
        }

        impl $name {
            #[must_use]
            pub fn unpack_flags(val: $repr) -> Vec<$name> {
                let mut flags = Vec::new();
                let mut val = val;
                let mut i = 0 as $repr;
                while val > 0 {
                    if val & 1 == 1 {
                        flags.push(unsafe { std::mem::transmute(i) });
                    }
                    val >>= 1;
                    i += 1;
                }
                flags
            }

            #[must_use]
            pub fn pack_flags(flags: &[Self]) -> $repr {
                flags.iter().fold(0, |acc, &flag| acc | (flag as $repr))
            }
        }
    };
}

flag! {
    u32,
    #[derive(OctData, Clone, Debug, Copy)]
    HollowGridFlag {
        Core = 1,
        CanMove = 2,
        Travelled = 4,
        ShowEventType = 8,
        ShowEventID = 16,
        CanTriggerEvent = 32,
        Visible = 64,
        VisibleAtGridAround = 128,
        VisibleByTriggerEvent = 256,
        SyncToClient = 512,
        Door = 1024,
        CanTriggerMultiTimes = 2048,
        TemporaryVisibleAtAround = 4096,
        Unlocked = 8192,
        Brighten = 16384,
        Guide = 32768,
        Target = 65536,
        BrightenOnlyVisible = 131072,
        Unstable = 262144,
    }
}

flag! {
    u8,
    #[derive(OctData, Clone, Debug, Copy)]
    HollowGridLink {
        None = 0,
        Up = 1,
        Down = 2,
        Right = 4,
        Left = 8,
        All = 15,
    }
}

#[derive(OctData, Clone, Debug)]
#[repr(i16)]
pub enum NodeState {
    All = 0,
    Locked = 1,
    Unlocked = 2,
    Finished = 3,
    ShowEvent = 4,
    Door = 5,
    Brighten = 6,
    Guide = 7,
    Target = 8,
    BrightenOnlyVisible = 9,
    Unstable = 10,
    EnumCount = 11,
}

#[derive(OctData, Clone, Debug)]
#[repr(i16)]
pub enum NodeVisible {
    All = 0,
    Visible = 1,
    VisibleAtGridAround = 2,
    VisibleByTriggerEvent = 3,
    TemporaryVisibleAtAround = 4,
    EnumCount = 5,
}

#[derive(OctData, Clone, Debug)]
#[repr(i16)]
pub enum HollowEventType {
    None = 0,
    All = 1,
    Begin = 10,
    End = 20,
    InteractEnd = 21,
    BattleEnd = 22,
    ChangeLevelInteract = 23,
    ChangeLevelFight = 24,
    Battle = 30,
    BattleNormal = 31,
    BattleElite = 32,
    BattleBoss = 33,
    Dialog = 40,
    DialogPositive = 41,
    DialogNegative = 42,
    DialogSpecial = 43,
}

#[derive(OctData, Clone, Debug)]
#[repr(i16)]
pub enum HollowShopCurrency {
    Coin = 1,
    Curse = 2,
    Random = 3,
}

#[derive(OctData, Clone, Debug)]
#[repr(i16)]
pub enum QuestState {
    Unlocked = 0,
    Ready = 10,
    InProgress = 1,
    ToFinish = 2,
    Finished = 3,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum QuestStatisticsType {
    ArrivedLevel = 1,
    EventCount = 2,
    CostTime = 3,
    KilledEnemyCount = 4,
    ArcanaCount = 5,
    TarotCardCount = 6,
    StaminaOverLevelTimes = 7,
    RebornTimes = 8,
    FinishedEventTypeCount = 9,
    FinishedEventIDCount = 10,
}

#[derive(OctData, Clone, Debug)]
#[repr(i16)]
pub enum System {
    HollowQuestUI = 0,
    VHSUI = 1,
    RoleUI = 2,
    SmithyUI = 3,
    PackageUI = 4,
    TeleportUI = 5,
    YorozuyaManualUI = 6,
    VHSStoreUI = 7,
    RamenUI = 8,
    WorkbenchUI = 9,
    GroceryUI = 10,
    VideoshopUI = 11,
    SwitchOfStoryMode = 12,
    SwitchOfQTE = 13,
    LineupSelect = 14,
    UseStoryMode = 15,
    UseManualQTEMode = 16,
}

#[derive(OctData, Clone, Debug)]
#[repr(i16)]
pub enum InteractTarget {
    NPC = 0,
    TriggerBox = 1,
}

#[derive(OctData, Clone, Debug)]
#[repr(i16)]
pub enum EventGraphOwnerType {
    Scene = 0,
    Section = 1,
    SceneUnit = 2,
    Hollow = 3,
}

#[derive(OctData, Clone, Debug)]
#[repr(i16)]
pub enum Operator {
    Enter = 0,
}
