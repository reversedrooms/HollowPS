use std::sync::atomic::{AtomicU64, Ordering};

pub struct UniqueIDManager(AtomicU64);

impl UniqueIDManager {
    const BASE_UID: u64 = 1000000;
    pub const fn new() -> Self {
        Self(AtomicU64::new(Self::BASE_UID))
    }

    pub fn next(&self) -> u64 {
        self.0.fetch_add(1, Ordering::SeqCst) + 1
    }
}
