mod battle;
mod beginner_procedure;
mod hollow;
mod login;
mod mail;
mod plot_play;
mod progression;
mod role;
mod world;
mod yorozuya;

pub use battle::*;
pub use beginner_procedure::*;
pub use hollow::*;
pub use login::*;
pub use mail::*;
pub use plot_play::*;
pub use progression::*;
pub use role::*;
pub use world::*;
pub use yorozuya::*;

use super::NetworkSession;
use anyhow::Result;
use paste::paste;
use protocol::*;

macro_rules! protocol_handlers {
    ($($name:ident;)*) => {
        pub trait ProtocolHandler {
            async fn on_message(session: &mut NetworkSession, protocol_id: u16, payload: Vec<u8>) -> Result<()> {
                use ::qwer::OctData;
                use ::tracing::Instrument;
                paste! {
                    match protocol_id {
                        $(::protocol::[<$name:snake:upper _ID>] => {
                                let arg = ::protocol::[<$name Arg>]::unmarshal_from(&mut &payload[..], 0)?;
                                let ret = [<on_$name:snake>](session, &arg)
                                    .instrument(tracing::info_span!(stringify!([<on_$name:snake>]), protocol_id = protocol_id))
                                    .await?;

                                session.send_rpc_ret(ret).await
                            }
                        )*
                        _ => {
                            tracing::warn!("Message with protocol id {protocol_id} wasn't handled!");
                            Ok(())
                        },
                    }
                }
            }

            async fn send_rpc_ret(&self, data: impl ::qwer::OctData) -> Result<()>;
        }
    };
}

protocol_handlers! {
    RpcLogin;
    PtcGetServerTimestamp;
    PtcPlayerOperation;
    RpcAdvanceBeginnerProcedure;
    RpcBattleReport;
    RpcBeginnerbattleBegin;
    RpcBeginnerbattleEnd;
    RpcCheckYorozuyaInfoRefresh;
    RpcCloseLevelChgTips;
    RpcDelNewMap;
    RpcEndBattle;
    RpcEnterWorld;
    RpcFinishActPerformShow;
    RpcFinishEventGraphPerformShow;
    RpcGetPlayerMails;
    RpcHollowMove;
    RpcInteractWithUnit;
    RpcKeepAlive;
    RpcLeaveCurDungeon;
    RpcModNickName;
    RpcPerformEnd;
    RpcPerformJump;
    RpcPerformTrigger;
    RpcRunEventGraph;
    RpcRunHollowEventGraph;
    RpcSavePosInMainCity;
    RpcStartHollowQuest;
}
