use std::time::Instant;

use crate::{CacheKey, MinaMesh};

impl MinaMesh {
  /// Checks the cache for a valid entry.
  pub fn get_from_cache(&self, key: CacheKey) -> Option<String> {
    if let Some(cached_entry) = self.cache.get(key.to_string().as_str()) {
      let (cached_value, timestamp) = &*cached_entry;
      if timestamp.elapsed() < self.cache_ttl {
        return Some(cached_value.clone());
      }
    }
    None
  }

  pub fn insert_into_cache(&self, key: CacheKey, value: String) {
    self.cache.insert(key.to_string(), (value, Instant::now()));
  }
}
