use parking_lot::RwLock;
use protocol::{ItemInfo, PlayerInfo};
use qwer::{phashmap, PropertyHashMap};
use std::sync::Arc;

use crate::game::{util, PlayerOperationResult};

use super::UniqueIDManager;

pub struct ItemManager {
    uid_mgr: Arc<UniqueIDManager>,
    player_info: Arc<RwLock<PlayerInfo>>,
}

impl ItemManager {
    pub fn new(uid_mgr: Arc<UniqueIDManager>, player_info: Arc<RwLock<PlayerInfo>>) -> Self {
        Self {
            uid_mgr,
            player_info,
        }
    }

    pub fn add_resource(&self, currency_id: i32, amount: i32) -> PlayerOperationResult<i32> {
        let mut player_info = self.player_info.write();

        for (uid, item) in player_info.items.as_mut().unwrap() {
            if let ItemInfo::Resource { id, count, .. } = item {
                if currency_id == *id {
                    *count += amount;

                    return PlayerOperationResult::with_changes(
                        *count,
                        PlayerInfo {
                            items: Some(PropertyHashMap::Modify {
                                to_add: vec![(*uid, item.clone())],
                                to_remove: Vec::new(),
                            }),
                            ..Default::default()
                        },
                    );
                }
            }
        }

        let uid = self.uid_mgr.next();
        let item = ItemInfo::Resource {
            uid,
            id: currency_id,
            count: amount,
            package: 3,
            first_get_time: util::cur_timestamp_ms(),
        };

        let items = player_info.items.as_mut().unwrap();
        items.insert(uid, item.clone());

        PlayerOperationResult::with_changes(
            amount,
            PlayerInfo {
                items: Some(PropertyHashMap::Modify {
                    to_add: vec![(uid, item)],
                    to_remove: Vec::new(),
                }),
                ..Default::default()
            },
        )
    }

    pub fn unlock_avatar(&self, id: i32) -> PlayerOperationResult<u64> {
        let uid = self.uid_mgr.next();

        let avatar = ItemInfo::Avatar {
            uid,
            id,
            count: 1,
            package: 3,
            first_get_time: util::cur_timestamp_ms(),
            star: 1,
            exp: 0,
            level: 1,
            rank: 1,
            unlocked_talent_num: 0,
            skills: phashmap![(2, 1), (1, 1), (0, 1), (3, 1), (4, 1)],
            is_custom_by_dungeon: true,
            robot_id: 0,
        };

        // Unlock & equip default weapon
        let weapon_uid = self.unlock_weapon(10012).take();
        self.equip_weapon(weapon_uid, uid);

        let mut player_info = self.player_info.write();
        let items = player_info.items.as_mut().unwrap();
        items.insert(uid, avatar.clone());

        PlayerOperationResult::with_changes(
            uid,
            PlayerInfo {
                items: Some(PropertyHashMap::Modify {
                    to_add: vec![
                        (uid, avatar),
                        (weapon_uid, items.get(&weapon_uid).unwrap().clone()),
                    ],
                    to_remove: vec![],
                }),
                ..Default::default()
            },
        )
    }

    pub fn unlock_weapon(&self, id: i32) -> PlayerOperationResult<u64> {
        let mut player_info = self.player_info.write();
        let items = player_info.items.as_mut().unwrap();

        let uid = self.uid_mgr.next();

        let weapon = ItemInfo::Weapon {
            uid,
            id,
            count: 1,
            package: 3,
            first_get_time: util::cur_timestamp_ms(),
            avatar_uid: 0,
            star: 0,
            exp: 0,
            level: 1,
            lock: 0,
            refine_level: 1,
        };

        items.insert(uid, weapon.clone());
        PlayerOperationResult::with_changes(
            uid,
            PlayerInfo {
                items: Some(PropertyHashMap::Modify {
                    to_add: vec![(uid, weapon)],
                    to_remove: Vec::new(),
                }),
                ..Default::default()
            },
        )
    }

    pub fn equip_weapon(
        &self,
        weapon_uid: u64,
        equip_avatar_uid: u64,
    ) -> PlayerOperationResult<bool> {
        let mut player_info = self.player_info.write();
        let items = player_info.items.as_mut().unwrap();

        let Some(ItemInfo::Weapon { avatar_uid, .. }) = items.get_mut(&weapon_uid) else {
            return PlayerOperationResult::ret(false);
        };

        *avatar_uid = equip_avatar_uid;

        PlayerOperationResult::with_changes(
            true,
            PlayerInfo {
                items: Some(PropertyHashMap::Modify {
                    to_add: vec![(weapon_uid, items.get(&weapon_uid).unwrap().clone())],
                    to_remove: Vec::new(),
                }),
                ..Default::default()
            },
        )
    }
}
