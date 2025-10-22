use rand::{Rng, thread_rng};
use std::collections::LinkedList;

pub struct SecureHashMap {
    capacity: usize,
    seed1: u64,
    seed2: u64,
    buckets: Vec<LinkedList<(String, String)>>,
    max_chain_length: usize,
    collision_threshold: u32,
    collision_count: u32,
    rehash_count: u32,
}

impl SecureHashMap {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        SecureHashMap {
            capacity: 16,
            seed1: rng.gen(),
            seed2: rng.gen(),
            buckets: vec![LinkedList::new(); 16],
            max_chain_length: 8,
            collision_threshold: 3,
            collision_count: 0,
            rehash_count: 0,
        }
    }

    fn hash(&self, key: &str) -> usize {
        let mut h = self.seed1;
        for c in key.chars() {
            h = self.sip_round(h, c as u64, self.seed2);
        }
        (h % self.capacity as u64) as usize
    }

    fn sip_round(&self, v: u64, m: u64, k: u64) -> u64 {
        let mut v = v.wrapping_add(m);
        v = v ^ k;
        v = v.rotate_left(13);
        v = v.wrapping_add(k);
        v
    }

    pub fn set(&mut self, key: &str, value: &str) {
        let mut idx = self.hash(key);
        if self.buckets[idx].len() >= self.max_chain_length {
            self.collision_count += 1;
            if self.collision_count >= self.collision_threshold {
                self.rehash_with_new_seed();
                // After rehashing, the index for the key might have changed
                idx = self.hash(key);
            }
        }

        for (k, v) in self.buckets[idx].iter_mut() {
            if k == key {
                *v = value.to_string();
                return;
            }
        }
        self.buckets[idx].push_back((key.to_string(), value.to_string()));
    }

    fn rehash_with_new_seed(&mut self) {
        self.rehash_count += 1;
        println!("🔄 Rehash #{} with new random seed", self.rehash_count);
        let mut rng = thread_rng();
        self.seed1 = rng.gen();
        self.seed2 = rng.gen();

        let old_buckets = std::mem::replace(&mut self.buckets, vec![LinkedList::new(); self.capacity]);
        self.collision_count = 0;

        for bucket in old_buckets {
            for (key, value) in bucket {
                // Re-insert directly into the new buckets without calling the public `set`
                let new_idx = self.hash(&key);
                self.buckets[new_idx].push_back((key, value));
            }
        }
    }
}
