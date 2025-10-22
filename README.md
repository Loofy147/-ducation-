# The Seven Missing Computational Fundamentals: A Rust Implementation

This repository provides production-ready Rust implementations and detailed research papers for seven fundamental principles of modern computing that are often overlooked in classical computer science.

This project is not just a collection of algorithms; it is a demonstration of how to build systems that are **robust, adaptive, secure, and intelligent** by design. It includes both a `verification_suite` to test each principle in isolation and a comprehensive `edge_simulation` to show how they work together in a realistic, production-style scenario.

---

## The Seven Fundamentals

1.  **[Time-Aware Computing](./docs/time-aware/README.md)**
    *   **Concept:** Algorithms that can trade deliberation time for result quality to meet real-world deadlines.
    *   **Implementation:** `AnytimeQuicksort`.

2.  **[Resource-Aware Computing](./docs/resource-aware/README.md)**
    *   **Concept:** Systems that perform multi-objective optimization across competing resources like CPU, energy, and memory.
    *   **Implementation:** A probabilistic `ResourceAwareScheduler`.

3.  **[Adversarial-First Design](./docs/adversarial-first/README.md)**
    *   **Concept:** Building systems that are secure by default by treating all external inputs as potentially malicious.
    *   **Implementation:** A `SecureHashMap` resistant to hash-flooding DoS attacks.

4.  **[Algebraic Composability](./docs/algebraic-composability/README.md)**
    *   **Concept:** Using formal algebraic structures (monoids) to guarantee that distributed data aggregation is provably correct.
    *   **Implementation:** A generic `Monoid` struct used for aggregating `TaskStats`.

5.  **[Uncertainty Quantification](./docs/uncertainty-quantification/README.md)**
    *   **Concept:** Modeling and propagating uncertainty through a system so it can "know what it doesn't know."
    *   **Implementation:** An `UncertainValue` struct that enables risk-aware decision-making.

6.  **[Self-Modifying Algorithms](./docs/self-modifying/README.md)**
    *   **Concept:** Algorithms that monitor their own performance and adapt their internal strategy to suit the current workload.
    *   **Implementation:** A `SelfOptimizingCache` that switches between LRU and LFU.

7.  **[Causal Reasoning](./docs/causal-reasoning/README.md)**
    *   **Concept:** Moving beyond statistical correlation to understand cause-and-effect relationships, avoiding common fallacies like Simpson's Paradox.
    *   **Implementation:** A simulation and analysis demonstrating Simpson's Paradox.

---

## Getting Started

This project is a standard Rust crate with two binaries.

### Running the Verification Suite

The `verification_suite` runs a series of tests to demonstrate each of the seven fundamentals in isolation.

```bash
cargo run --bin verification_suite
```

### Running the Edge Server Simulation

The `edge_simulation` is a comprehensive, integrated simulation that shows all seven principles working together in a realistic, production-style scenario.

```bash
cargo run --bin edge_simulation
```
