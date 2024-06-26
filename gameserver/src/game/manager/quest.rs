use std::sync::Arc;

use parking_lot::RwLock;
use qwer::PropertyDoubleKeyHashMap;

use crate::game::PlayerOperationResult;

use super::UniqueIDManager;
use protocol::*;

pub struct QuestManager {
    uid_mgr: Arc<UniqueIDManager>,
    player: Arc<RwLock<PlayerInfo>>,
}

impl QuestManager {
    pub fn new(uid_mgr: Arc<UniqueIDManager>, player: Arc<RwLock<PlayerInfo>>) -> Self {
        Self { uid_mgr, player }
    }

    pub fn add_world_quest(&self, quest: QuestInfo) -> PlayerOperationResult<u64> {
        let mut world_quest_collection_uid = self
            .player
            .read()
            .quest_data
            .as_ref()
            .unwrap()
            .world_quest_collection_uid
            .unwrap();

        if world_quest_collection_uid == 0 {
            world_quest_collection_uid = self.uid_mgr.next();
            self.player
                .write()
                .quest_data
                .as_mut()
                .unwrap()
                .world_quest_collection_uid
                .replace(world_quest_collection_uid);
        }

        self.add_quest_to_collection(world_quest_collection_uid, quest)
    }

    pub fn add_quest_to_collection(
        &self,
        collection_uid: u64,
        mut quest: QuestInfo,
    ) -> PlayerOperationResult<u64> {
        let mut player = self.player.write();
        let quest_data = player.quest_data.as_mut().unwrap();
        quest.set_collection_uid(collection_uid);

        quest_data
            .quests
            .as_mut()
            .unwrap()
            .insert(collection_uid, quest.get_id(), quest.clone());

        PlayerOperationResult::with_changes(
            collection_uid,
            PlayerInfo {
                quest_data: Some(QuestData {
                    quests: Some(PropertyDoubleKeyHashMap::Modify {
                        to_add: vec![(collection_uid, quest.get_id(), quest)],
                        to_remove: Vec::new(),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
    }
}
