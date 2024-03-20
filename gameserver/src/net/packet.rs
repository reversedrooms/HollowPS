use anyhow::Result;
use paste::paste;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tracing::Instrument;

use protocol::*;
use qwer::ProtocolHeader;

use super::handlers::*;
use super::NetworkSession;

pub struct Packet {
    pub to_channel: u16,
    pub header: ProtocolHeader,
    pub body: Vec<u8>,
}

pub struct RequestBody {
    pub protocol_id: u16,
    pub payload: Vec<u8>,
}

pub struct ResponseBody {
    pub middleware_id: u16,
    pub middleware_error_code: u16,
    pub payload: Vec<u8>,
}

impl Packet {
    pub async fn read(stream: &mut TcpStream) -> std::io::Result<Self> {
        let to_channel = stream.read_u16_le().await?;
        let body_size = stream.read_u32_le().await? as usize;
        let header_size = stream.read_u16_le().await? as usize;

        let mut header = vec![0; header_size];
        stream.read_exact(&mut header).await?;

        let mut body = vec![0; body_size];
        stream.read_exact(&mut body).await?;

        Ok(Self {
            to_channel,
            header: header.into(),
            body,
        })
    }
}

impl From<Vec<u8>> for RequestBody {
    fn from(value: Vec<u8>) -> Self {
        let protocol_id = u16::from_le_bytes(value[0..2].try_into().unwrap());
        let payload_length = u32::from_be_bytes(value[2..6].try_into().unwrap()) as usize;
        let payload = value[6..payload_length + 6].to_vec();

        Self {
            protocol_id,
            payload,
        }
    }
}

impl From<RequestBody> for Vec<u8> {
    fn from(value: RequestBody) -> Self {
        let mut out = Self::new();

        out.extend(value.protocol_id.to_le_bytes());
        out.extend((value.payload.len() as u32).to_be_bytes());
        out.extend(value.payload);

        out
    }
}

impl From<ResponseBody> for Vec<u8> {
    fn from(value: ResponseBody) -> Self {
        let mut out = Self::with_capacity(4 + value.payload.len());
        out.extend(value.middleware_id.to_le_bytes());
        out.extend(value.middleware_error_code.to_le_bytes());
        out.extend(value.payload);

        out
    }
}

macro_rules! trait_handler {
    ($($name:ident $protocol_id:expr;)*) => {
        #[allow(dead_code)]
        #[allow(unused_variables)]
        pub trait PacketHandler {
            $(
                paste! {
                    #[tracing::instrument(skip(session))]
                    async fn [<on_$name:snake>](session: &mut NetworkSession, arg: &$name) -> Result<()> {
                        [<on_$name:snake>](session, arg).await
                    }
                }
            )*

            async fn on_message(session: &mut NetworkSession, protocol_id: u16, payload: Vec<u8>) -> Result<()> {
                use ::qwer::OctData;
                match protocol_id {
                    $(
                        $protocol_id => {
                            let arg = $name::unmarshal_from(&mut &payload[..], 0)?;
                            paste! {
                                Self::[<on_$name:snake>](session, &arg)
                                    .instrument(tracing::info_span!(stringify!([<on_$name:snake>]), protocol_id = protocol_id))
                                    .await
                            }
                        }
                    )*
                    _ => {
                        tracing::warn!("Unknown protocol id: {protocol_id}");
                        Ok(())
                    },
                }
            }
        }
    };
}

trait_handler! {
    RpcLoginArg 100;
    // PtcAbilityPopText 239;
    // PtcAccountInfoChanged 102;
    // PtcAvatarMapChanged 246;
    // PtcBeforeGoToHollowLevel 145;
    // PtcCardDisable 217;
    // PtcChallengeQuestFinished 193;
    // PtcClientCommon 110;
    // PtcConfigUpdated 277;
    // PtcDungeonQuestFinished 148;
    // PtcDungeonQuestPrepareToFinish 136;
    // PtcEnterScene 118;
    // PtcEnterSceneBegin 200;
    // PtcEnterSceneEnd 224;
    // PtcEnterSection 243;
    // PtcFairyInfoChanged 134;
    // PtcFunctionSwitchMask 279;
    PtcGetServerTimestampArg 204;
    // PtcGoToHollowLevel 154;
    // PtcHollowBlackout 268;
    // PtcHollowGlobalEvent 138;
    // PtcHollowGrid 114;
    // PtcHollowPushBack 284;
    // PtcHollowQuestUnlockedByMainCityQuest 201;
    // PtcHpOrStressChanged 226;
    // PtcItemChanged 117;
    // PtcKickPlayer 184;
    // PtcPauseMainCityTime 116;
    // PtcPlayerInfoChangedArg 101;
    // PtcPlayerMailsReceived 222;
    // PtcPlayerMailsRemoved 225;
    PtcPlayerOperationArg 203;
    // PtcPopupWindow 206;
    // PtcPosition 176;
    // PtcPositionInHollowChanged 141;
    // PtcPrepareSection 115;
    // PtcPreventAddiction 270;
    // PtcPropertyChanged 129;
    // PtcQuestUnlocked 158;
    // PtcReceivedChatMessage_Player2Client 165;
    // PtcScenePropertyChanged 128;
    // PtcShowCardGenreTips 276;
    // PtcShowTips 207;
    // PtcShowUnlockIDTips 278;
    // PtcStaminaOverLevelPunish 147;
    // PtcSyncEventInfo 177;
    // PtcSyncHollowEventInfo 210;
    // PtcSyncHollowGridMaps 124;
    // PtcSyncSceneTime 249;
    // PtcSyncSceneUnit 180;
    // PtcTransformToHollowGrid 144;
    // PtcUnlock 196;
    // RpcAFKHollowQuest 241;
    RpcAdvanceBeginnerProcedureArg 171;
    // RpcAvatarAdvance 111;
    // RpcAvatarLevelUp 107;
    // RpcAvatarSkillLevelUp 197;
    // RpcAvatarStarUp 108;
    // RpcAvatarUnlockTalent 199;
    // RpcAwardAllPlayerMail 257;
    // RpcAwardPlayerMail 256;
    // RpcBattleRebegin 286;
    RpcBattleReportArg 125;
    // RpcBeginArchiveBattleQuest 137;
    RpcBeginnerbattleBeginArg 258;
    RpcBeginnerbattleEndArg 285;
    // RpcBeginnerbattleRebegin 250;
    // RpcBuyAutoRecoveryItem 167;
    // RpcBuyVHSCollection 269;
    RpcCheckYorozuyaInfoRefreshArg 245;
    // RpcClickHollowSystem 282;
    RpcCloseLevelChgTipsArg 244;
    // RpcCreatePlayer 104;
    // RpcDebugPay 216;
    RpcDelNewMapArg 287;
    // RpcDelNewRamen 228;
    // RpcDressEquipment 112;
    // RpcEatRamen 283;
    RpcEndBattleArg 251;
    // RpcEndSlotMachine 186;
    // RpcEnterSection 175;
    RpcEnterWorldArg 105;
    // RpcEquipDecompose 170;
    // RpcEquipGacha 169;
    // RpcEquipLock 172;
    // RpcEquipmentLevelUp 130;
    // RpcEquipmentStarUp 131;
    RpcFinishACTPerformShowArg 185;
    // RpcFinishBlackout 267;
    RpcFinishEventGraphPerformShowArg 187;
    // RpcFinishGraphInClient 146;
    // RpcGMCommand 113;
    // RpcGacha 173;
    // RpcGetArchiveReward 166;
    // RpcGetAuthKey 280;
    // RpcGetChatHistory_Client2Player 159;
    RpcGetPlayerMailsArg 221;
    // RpcGetShopInfo 122;
    // RpcGetYorozuyaInfo 182;
    // RpcGiveUpDungeonQuest 142;
    // RpcHollowChangeAffix 143;
    RpcHollowMoveArg 248;
    // RpcHollowShopping 213;
    RpcInteractWithUnitArg 181;
    // RpcItemConvert 281;
    RpcKeepAliveArg 149;
    RpcLeaveCurDungeonArg 140;
    // RpcLeaveWorld 190;
    // RpcLogin 100;
    // RpcLogout 103;
    // RpcMakeChoiceOfEvent 214;
    // RpcMakeInitiativeItem 234;
    RpcModNickNameArg 215;
    // RpcOpenVHSStore 135;
    RpcPerformEndArg 255;
    RpcPerformJumpArg 254;
    RpcPerformTriggerArg 253;
    // RpcPrepareNextHollowEnd 252;
    // RpcReadPlayerMail 263;
    // RpcReceiveVHSStoreReward 227;
    // RpcReenterWorld 150;
    // RpcRefreshShop 237;
    // RpcRefreshVHSTrending 235;
    // RpcRemoveHollowCurse 229;
    // RpcRemovePlayerMailsFromClient 264;
    RpcRunEventGraphArg 179;
    RpcRunHollowEventGraphArg 211;
    RpcSavePosInMainCityArg 202;
    // RpcSelectChallenge 236;
    // RpcSelectVHSCollection 219;
    // RpcSendChatMessage_Client2Player 163;
    // RpcSetBGM 273;
    // RpcSetMainCityObjectState 274;
    // RpcSetPlayerMailOld 265;
    // RpcShopping 230;
    RpcStartHollowQuestArg 183;
    // RpcSwitchHollowRank 198;
    // RpcUndressEquipment 109;
    // RpcUseInitiativeItem 220;
    // RpcWeaponDecompose 120;
    // RpcWeaponDress 132;
    // RpcWeaponLevelUp 126;
    // RpcWeaponLock 119;
    // RpcWeaponRefine 232;
    // RpcWeaponStarUp 127;
    // RpcWeaponUnDress 139;
    // RpcYorozuyaManualReceiveReward 168;
}
