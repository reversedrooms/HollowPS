use std::sync::Arc;

use atomic_refcell::AtomicRefCell;
use protocol::*;
use qwer::PropertyHashSet;

use crate::game::PlayerOperationResult;

pub struct UnlockManager {
    player: Arc<AtomicRefCell<PlayerInfo>>,
}

impl UnlockManager {
    pub fn new(player: Arc<AtomicRefCell<PlayerInfo>>) -> Self {
        Self { player }
    }

    pub fn unlock(&self, unlock_id: i32) -> PlayerOperationResult<PtcUnlockArg> {
        let mut player = self.player.borrow_mut();
        let unlock_info = player.unlock_info.as_mut().unwrap();

        unlock_info
            .unlocked_list
            .as_mut()
            .unwrap()
            .insert(unlock_id);

        PlayerOperationResult::with_changes(
            PtcUnlockArg { unlock_id },
            PlayerInfo {
                unlock_info: Some(UnlockInfo {
                    unlocked_list: Some(PropertyHashSet::Modify {
                        to_add: vec![unlock_id],
                        to_remove: Vec::new(),
                    }),
                    condition_progress: None,
                }),
                ..Default::default()
            },
        )
    }
}
