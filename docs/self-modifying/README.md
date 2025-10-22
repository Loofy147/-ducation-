# Paper 6: Self-Modifying Algorithms

**A Framework for Systems That Learn and Adapt to Their Environment**

**Version:** 1.0
**Status:** Complete

---

## 1. Abstract

Classical algorithms are static; their logic is fixed at compile time. Production systems, however, operate in dynamic environments where workload patterns, data distributions, and resource availability can change dramatically. This paper introduces **Self-Modifying Algorithms**, a paradigm for building systems that monitor their own performance and dynamically adapt their internal logic to optimize for the current environment. We explore the theory of **adaptive and online algorithms**, present a practical implementation of a `SelfOptimizingCache` in Rust, and demonstrate its ability to switch its caching strategy in response to a changing workload in our `edge_simulation`.

---

## 2. The Missing Fundamental: The Problem of Static Design

An algorithm is typically designed and optimized for a specific, assumed workload. For example:
-   A caching system might use a Least Recently Used (LRU) policy, assuming that recently accessed items are likely to be accessed again.
-   A database query planner might choose a hash join, assuming a uniform distribution of keys.

These assumptions are fragile. If the workload changes, a once-optimal algorithm can become highly inefficient:
-   If a workload involves repeatedly scanning a small set of "hot" items, a Least Frequently Used (LFU) policy would be far superior to LRU.
-   If the data distribution is skewed, the query planner's choice may be disastrous.

The missing fundamental is a design principle where the system itself is responsible for **detecting changes in its environment and adapting its own algorithms** to maintain optimal performance.

---

## 3. Foundational Research: Adaptive and Online Algorithms

The concept of algorithms that adapt to their input is a core area of computer science research.
-   **Online Algorithms:** These algorithms process their input piece-by-piece in a serial fashion, making decisions at each step without knowledge of the future. The goal is to maintain a competitive ratio against an optimal offline algorithm that can see the entire input sequence in advance.
-   **Adaptive Algorithms:** This is a broader class of algorithms that can change their behavior based on observations of their input or performance. Research in "cache-adaptive" algorithms, for example, focuses on systems that can perform optimally even when the amount of available memory changes dynamically.

Our `SelfOptimizingCache` is a practical example of an adaptive, online algorithm. It makes a decision at each step (what to cache) and adapts its long-term strategy based on its observed performance (the hit rate).

---

## 4. Rust Implementation: `SelfOptimizingCache`

Our `SelfOptimizingCache` is designed to demonstrate this adaptive principle in a clear, understandable way.

### Core Mechanism

The struct maintains two internal caching strategies: a `LinkedHashMap` for LRU and a combination of a `HashMap` and another `LinkedHashMap` for LFU. It also tracks its own performance via `hits` and `misses`.

The core of its adaptive logic resides in the `adapt_strategy` method:

```rust
// from src/self_modifying.rs
fn adapt_strategy(&mut self) {
    if (self.hits + self.misses) >= 100 { // Re-evaluate every 100 operations
        let hit_rate = self.hits as f64 / (self.hits + self.misses) as f64;

        // Choose a new strategy based on the observed hit rate
        let new_strategy = if hit_rate > 0.6 { CacheStrategy::LFU } else { CacheStrategy::LRU };

        if new_strategy != self.strategy {
            println!("Adapting strategy to {:?}", new_strategy);
            self.migrate_cache(&new_strategy); // Migrate data to the new structure
            self.strategy = new_strategy;
        }

        // Reset counters for the next evaluation window
        self.hits = 0;
        self.misses = 0;
    }
}
```
This feedback loop—**Measure, Analyze, Adapt**—is the essence of a self-modifying algorithm. When the strategy changes, the `migrate_cache` function is called to transfer the existing cached data into the data structures for the new strategy, ensuring a seamless transition.

---

## 5. Verification and Demonstration

The `edge_simulation` is designed to prove that this adaptation works in practice. The simulation runs in two phases, each designed to favor a different caching strategy:

1.  **Phase 1 (LRU-favoring):** The simulation requests a series of unique keys (`scanning_task_1`, `scanning_task_2`, etc.). This "scanning" workload has a low hit rate and is best served by an LRU policy, which correctly evicts the old, unused items.
2.  **Phase 2 (LFU-favoring):** The simulation requests the *same key* (`repeated_hotspot_task`) over and over. This workload has a very high hit rate and is best served by an LFU policy, which would recognize the high frequency of this one item.

The simulation output clearly shows the cache adapting. After the LFU-favoring workload, the console prints:
`Adapting strategy to LFU`

And after the subsequent scanning workload, it prints:
`Adapting strategy to LRU`

This demonstrates that the system is not static; it is actively observing its environment and modifying its own logic to achieve better performance.

---

### References
1.  Bender, M. A., et al. 2015. *Cache-Adaptive Algorithms*. In *Proceedings of the 27th ACM Symposium on Parallelism in Algorithms and Architectures*.
2.  Karp, R. M. 1992. *On-Line Algorithms Versus Off-Line Algorithms: How Much is it Worth to Know the Future?*. In *Proceedings of the IFIP 12th World Computer Congress*.
