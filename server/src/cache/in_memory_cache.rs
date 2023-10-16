use std::time::Duration;

use common::{
    challenge::Challenge,
    proto::ClientId,
};
use moka::sync::Cache;

use crate::cache::challenge_cache::ChallengeCache;

#[derive(Clone)]
pub struct InMemoryCache {
    inner: Cache<ClientId, Challenge>,
}

impl InMemoryCache {
    pub fn new(capacity: u64) -> InMemoryCache {
        InMemoryCache {
            inner: Cache::builder()
                .max_capacity(capacity)
                .time_to_live(Duration::from_secs(60))
                .build(),
        }
    }
}

impl ChallengeCache for InMemoryCache {
    fn get(&self, key: &ClientId) -> Option<Challenge> {
        self.inner.get(key)
    }

    fn set(&self, key: ClientId, value: Challenge) {
        self.inner.insert(key, value)
    }
}
