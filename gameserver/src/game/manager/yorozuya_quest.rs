use std::collections::HashSet;
use std::sync::Arc;

use atomic_refcell::AtomicRefCell;
use qwer::{PropertyDoubleKeyHashMap, PropertyHashSet};

use crate::game::PlayerOperationResult;

use protocol::*;

pub struct YorozuyaQuestManager {
    player: Arc<AtomicRefCell<PlayerInfo>>,
}

impl YorozuyaQuestManager {
    pub fn new(player: Arc<AtomicRefCell<PlayerInfo>>) -> Self {
        Self { player }
    }

    pub fn add_hollow_quest(
        &self,
        yorozuya_collection_id: i32,
        hollow_quest_type: HollowQuestType,
        id: i32,
    ) -> PlayerOperationResult<i32> {
        let mut player = self.player.borrow_mut();
        let yorozuya = player.yorozuya_info.as_mut().unwrap();
        let hollow_quests = yorozuya.hollow_quests.as_mut().unwrap();

        let updated_set = {
            if let Some(quests) = hollow_quests.get_mut(&yorozuya_collection_id, &hollow_quest_type)
            {
                quests.insert(id);
                let PropertyHashSet::Base(set) = quests else {
                    return PlayerOperationResult::ret(yorozuya_collection_id);
                };
                set.clone()
            } else {
                let set = HashSet::from([id]);
                hollow_quests.insert(
                    yorozuya_collection_id,
                    hollow_quest_type,
                    PropertyHashSet::Base(set.clone()),
                );

                set
            }
        };

        PlayerOperationResult::with_changes(
            yorozuya_collection_id,
            PlayerInfo {
                yorozuya_info: Some(YorozuyaInfo {
                    hollow_quests: Some(PropertyDoubleKeyHashMap::Modify {
                        to_add: vec![(
                            yorozuya_collection_id,
                            hollow_quest_type,
                            PropertyHashSet::Base(updated_set),
                        )],
                        to_remove: Vec::new(),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
    }
}
