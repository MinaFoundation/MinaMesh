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
    match key {
      CacheKey::NetworkId => {
        // Store permanently without eviction
        self.cache.insert(key.to_string(), (value, Instant::now()));
      }
      CacheKey::Transaction(_) => {
        // Count only Transaction entries by checking prefix "txn_"
        let transaction_count = self.cache.iter().filter(|entry| entry.key().to_string().starts_with("txn_")).count();

        if transaction_count >= self.cache_tx_size {
          // Find the oldest transaction entry
          let oldest_key = self
            .cache
            .iter()
            .filter_map(|entry| {
              let key_str = entry.key().to_string();
              if key_str.starts_with("txn_") {
                Some((entry.key().clone(), entry.value().1))
              } else {
                None
              }
            })
            .min_by_key(|(_, timestamp)| timestamp.elapsed())
            .map(|(key, _)| key);

          // Remove the oldest transaction entry
          if let Some(key_to_remove) = oldest_key {
            self.cache.remove(&key_to_remove);
          }
        }

        // Insert new transaction
        self.cache.insert(key.to_string(), (value, Instant::now()));
      }
    }
  }

  pub fn is_transaction_cached(&self, signed_tx_str: &str) -> bool {
    self.cache.get(&CacheKey::Transaction(signed_tx_str.to_string()).to_string()).is_some()
  }

  pub fn cache_transaction(&self, signed_tx_str: &str) {
    self.insert_into_cache(CacheKey::Transaction(signed_tx_str.to_string()), "".to_string());
  }
}
