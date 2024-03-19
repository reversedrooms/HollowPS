use std::sync::atomic::{AtomicU64, Ordering};

const BASE_UID: u64 = 1000000;

pub struct UniqueIDManager {
    uid_counter: AtomicU64,
}

impl UniqueIDManager {
    pub const fn new() -> Self {
        Self {
            uid_counter: AtomicU64::new(BASE_UID),
        }
    }

    pub fn next(&self) -> u64 {
        let uid = self.uid_counter.load(Ordering::SeqCst) + 1;
        self.uid_counter.store(uid, Ordering::SeqCst);

        uid
    }
}
