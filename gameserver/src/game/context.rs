use std::sync::Arc;

use anyhow::Result;
use atomic_refcell::AtomicRefCell;
use protocol::{PlayerInfo, PtcPlayerInfoChangedArg};

use crate::net::NetworkSession;

use super::manager::{
    DungeonManager, HollowGridManager, ItemManager, QuestManager, SceneUnitManager,
    UniqueIDManager, UnlockManager, YorozuyaQuestManager,
};

pub struct GameContext {
    pub player: Arc<AtomicRefCell<PlayerInfo>>,
    pub uid_manager: Arc<AtomicRefCell<UniqueIDManager>>,
    pub item_manager: Arc<AtomicRefCell<ItemManager>>,
    pub dungeon_manager: Arc<AtomicRefCell<DungeonManager>>,
    pub quest_manager: Arc<AtomicRefCell<QuestManager>>,
    pub scene_unit_manager: Arc<AtomicRefCell<SceneUnitManager>>,
    pub hollow_grid_manager: Arc<AtomicRefCell<HollowGridManager>>,
    pub unlock_manager: Arc<AtomicRefCell<UnlockManager>>,
    pub yorozuya_quest_manager: Arc<AtomicRefCell<YorozuyaQuestManager>>,
}

impl GameContext {
    pub fn new(player: Arc<AtomicRefCell<PlayerInfo>>) -> Self {
        let uid_manager = Arc::new(AtomicRefCell::new(UniqueIDManager::new()));

        Self {
            player: player.clone(),
            uid_manager: uid_manager.clone(),
            item_manager: Arc::new(AtomicRefCell::new(ItemManager::new(
                uid_manager.clone(),
                player.clone(),
            ))),
            dungeon_manager: Arc::new(AtomicRefCell::new(DungeonManager::new(
                uid_manager.clone(),
                player.clone(),
            ))),
            quest_manager: Arc::new(AtomicRefCell::new(QuestManager::new(
                uid_manager.clone(),
                player.clone(),
            ))),
            scene_unit_manager: Arc::new(AtomicRefCell::new(SceneUnitManager::new(uid_manager))),
            hollow_grid_manager: Arc::new(AtomicRefCell::new(HollowGridManager::new(
                player.clone(),
            ))),
            unlock_manager: Arc::new(AtomicRefCell::new(UnlockManager::new(player.clone()))),
            yorozuya_quest_manager: Arc::new(AtomicRefCell::new(YorozuyaQuestManager::new(player))),
        }
    }
}

pub struct PlayerOperationResult<T>
where
    T: Send + Sync,
{
    result: T,
    player_info_changes: Option<PlayerInfo>,
}

impl<T> PlayerOperationResult<T>
where
    T: Send + Sync,
{
    pub const fn unwrap(&self) -> &T {
        &self.result
    }

    pub async fn send_changes(&mut self, session: &NetworkSession) -> Result<&T> {
        if self.player_info_changes.is_some() {
            let ptc_player_info_changed = PtcPlayerInfoChangedArg {
                player_uid: session.get_player_uid(),
                player_info: self.player_info_changes.take().unwrap(),
            };

            session.send_rpc_arg(101, &ptc_player_info_changed).await?;
        }

        Ok(self.unwrap())
    }

    pub const fn ret(result: T) -> Self {
        Self {
            result,
            player_info_changes: None,
        }
    }

    pub const fn with_changes(result: T, player_info_changes: PlayerInfo) -> Self {
        Self {
            result,
            player_info_changes: Some(player_info_changes),
        }
    }
}
