use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;

/// An enumeration of possible cache eviction strategies.
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CacheStrategy {
    /// Least Recently Used.
    LRU,
    /// Least Frequently Used.
    LFU,
}

/// A cache that can dynamically switch its eviction strategy based on workload patterns.
pub struct SelfOptimizingCache<K, V> {
    capacity: usize,
    strategy: CacheStrategy,
    lru_map: LinkedHashMap<K, V>,
    lfu_map: HashMap<K, (V, usize)>,
    lfu_freq: LinkedHashMap<usize, Vec<K>>,
    hits: u64,
    misses: u64,
}

impl<K: Eq + std::hash::Hash + Clone, V: Clone> SelfOptimizingCache<K, V> {
    /// Creates a new `SelfOptimizingCache` with the given capacity.
    pub fn new(capacity: usize) -> Self {
        SelfOptimizingCache {
            capacity,
            strategy: CacheStrategy::LRU,
            lru_map: LinkedHashMap::new(),
            lfu_map: HashMap::new(),
            lfu_freq: LinkedHashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    /// Retrieves a value from the cache.
    pub fn get(&mut self, key: &K) -> Option<V> {
        let result = match self.strategy {
            CacheStrategy::LRU => self.lru_get(key),
            CacheStrategy::LFU => self.lfu_get(key),
        };

        if result.is_some() {
            self.hits += 1;
        } else {
            self.misses += 1;
        }
        self.adapt_strategy();
        result
    }

    /// Inserts a key-value pair into the cache.
    pub fn put(&mut self, key: K, value: V) {
        match self.strategy {
            CacheStrategy::LRU => self.lru_put(key, value),
            CacheStrategy::LFU => self.lfu_put(key, value),
        }
        self.adapt_strategy();
    }

    fn lru_get(&mut self, key: &K) -> Option<V> {
        self.lru_map.get_refresh(key).map(|v| v.clone())
    }

    fn lru_put(&mut self, key: K, value: V) {
        if self.lru_map.len() >= self.capacity {
            self.lru_map.pop_front();
        }
        self.lru_map.insert(key, value);
    }

    fn lfu_get(&mut self, key: &K) -> Option<V> {
        let (value, freq) = match self.lfu_map.get_mut(key) {
            Some((value, freq)) => (value.clone(), *freq),
            None => return None,
        };

        self.update_freq(key.clone(), freq);
        if let Some((_, f)) = self.lfu_map.get_mut(key) {
            *f += 1;
        }
        Some(value)
    }

    fn lfu_put(&mut self, key: K, value: V) {
        if self.lfu_map.len() >= self.capacity {
            if let Some((_freq, keys)) = self.lfu_freq.iter_mut().next() {
                if let Some(key_to_evict) = keys.pop() {
                    self.lfu_map.remove(&key_to_evict);
                }
                if keys.is_empty() {
                    self.lfu_freq.pop_front();
                }
            }
        }
        self.lfu_map.insert(key.clone(), (value, 1));
        self.lfu_freq.entry(1).or_default().push(key);
    }

    fn update_freq(&mut self, key: K, freq: usize) {
        if let Some(keys) = self.lfu_freq.get_mut(&freq) {
            keys.retain(|k| k != &key);
            if keys.is_empty() {
                self.lfu_freq.remove(&freq);
            }
        }
        self.lfu_freq
            .entry(freq + 1)
            .or_default()
            .push(key);
    }

    fn adapt_strategy(&mut self) {
        if (self.hits + self.misses) >= 100 {
            let hit_rate = self.hits as f64 / (self.hits + self.misses) as f64;
            let new_strategy = if hit_rate > 0.6 {
                CacheStrategy::LFU
            } else {
                CacheStrategy::LRU
            };
            if new_strategy != self.strategy {
                println!("Adapting strategy to {:?}", new_strategy);
                self.migrate_cache(&new_strategy);
                self.strategy = new_strategy;
            }
            self.hits = 0;
            self.misses = 0;
        }
    }

    fn migrate_cache(&mut self, new_strategy: &CacheStrategy) {
        match new_strategy {
            CacheStrategy::LFU => {
                let data_to_migrate: Vec<_> = self
                    .lru_map
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                self.lru_map.clear();
                for (key, value) in data_to_migrate {
                    self.lfu_put(key, value);
                }
            }
            CacheStrategy::LRU => {
                let data_to_migrate: Vec<_> = self
                    .lfu_map
                    .iter()
                    .map(|(k, (v, _))| (k.clone(), v.clone()))
                    .collect();
                self.lfu_map.clear();
                self.lfu_freq.clear();
                for (key, value) in data_to_migrate {
                    self.lru_put(key, value);
                }
            }
        }
    }

    /// Returns the current `CacheStrategy`.
    pub fn get_strategy(&self) -> &CacheStrategy {
        &self.strategy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_self_modification() {
        let mut cache = SelfOptimizingCache::new(10);
        assert_eq!(*cache.get_strategy(), CacheStrategy::LRU, "Initial strategy should be LRU");

        // Fill the cache
        for i in 0..10 {
            cache.put(i, i);
        }

        // Simulate a workload that favors LFU (high hit rate)
        for _ in 0..100 {
            cache.get(&0);
        }

        assert_eq!(*cache.get_strategy(), CacheStrategy::LFU, "Strategy should adapt to LFU");

        // Simulate a workload that favors LRU (low hit rate)
        for i in 0..100 {
            cache.get(&(i % 20)); // Access a wider range of keys
        }
        assert_eq!(*cache.get_strategy(), CacheStrategy::LRU, "Strategy should adapt back to LRU");
    }
}
