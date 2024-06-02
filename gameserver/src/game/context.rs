use std::sync::Arc;

use anyhow::Result;
use parking_lot::RwLock;
use protocol::{PlayerInfo, PtcPlayerInfoChangedArg};

use crate::net::NetworkSession;

use super::manager::{
    DungeonManager, HollowGridManager, ItemManager, QuestManager, SceneUnitManager,
    UniqueIDManager, UnlockManager, YorozuyaQuestManager,
};

pub struct GameContext {
    #[allow(unused)]
    pub uid_manager: Arc<UniqueIDManager>,
    pub item_manager: Arc<ItemManager>,
    pub dungeon_manager: Arc<DungeonManager>,
    pub quest_manager: Arc<QuestManager>,
    pub scene_unit_manager: Arc<SceneUnitManager>,
    pub hollow_grid_manager: Arc<HollowGridManager>,
    pub unlock_manager: Arc<UnlockManager>,
    pub yorozuya_quest_manager: Arc<YorozuyaQuestManager>,
}

impl GameContext {
    pub fn new(player: Arc<RwLock<PlayerInfo>>) -> Self {
        let uid_manager = Arc::new(UniqueIDManager::new());

        Self {
            uid_manager: uid_manager.clone(),
            item_manager: Arc::new(ItemManager::new(uid_manager.clone(), player.clone())),
            dungeon_manager: Arc::new(DungeonManager::new(uid_manager.clone(), player.clone())),
            quest_manager: Arc::new(QuestManager::new(uid_manager.clone(), player.clone())),
            scene_unit_manager: Arc::new(SceneUnitManager::new(uid_manager)),
            hollow_grid_manager: Arc::new(HollowGridManager::new(player.clone())),
            unlock_manager: Arc::new(UnlockManager::new(player.clone())),
            yorozuya_quest_manager: Arc::new(YorozuyaQuestManager::new(player)),
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
                player_uid: session.player_uid().raw(),
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
