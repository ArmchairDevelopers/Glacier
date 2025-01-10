use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, OwnedRwLockReadGuard, OwnedRwLockWriteGuard, RwLock};

#[derive(Default)]
pub struct StringLock {
    locks: Mutex<HashMap<String, Arc<RwLock<()>>>>,
}

impl StringLock {
    pub fn new() -> Self {
        Self {
            locks: Mutex::new(HashMap::new()),
        }
    }

    /// Acquires a read lock for the given string.
    pub async fn lock_read(&self, key: &str) -> OwnedRwLockReadGuard<()> {
        let lock = self.get_or_create_lock(key).await;
        lock.read_owned().await
    }

    /// Acquires a write lock for the given string.
    pub async fn lock_write(&self, key: &str) -> OwnedRwLockWriteGuard<()> {
        let lock = self.get_or_create_lock(key).await;
        lock.write_owned().await
    }

    /// Ensures a lock exists for the given string and returns it.
    async fn get_or_create_lock(&self, key: &str) -> Arc<RwLock<()>> {
        let mut locks = self.locks.lock().await;
        locks
            .entry(key.to_string())
            .or_insert_with(|| Arc::new(RwLock::new(())))
            .clone()
    }

    /// Cleans up the lock for a string if no other references exist.
    /// Call this after the lock is released, if needed.
    pub async fn cleanup_lock(&self, key: &str) {
        let mut locks = self.locks.lock().await;
        if let Some(lock) = locks.get(key) {
            if Arc::strong_count(lock) == 1 {
                locks.remove(key);
            }
        }
    }
}
