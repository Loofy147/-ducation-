# Paper 5: Uncertainty Quantification

**A Framework for Building Systems That Know What They Don't Know**

**Version:** 1.0
**Status:** Complete

---

## 1. Abstract

Classical algorithms operate on deterministic inputs, producing exact outputs. Real-world systems, however, are rife with noise, incomplete data, and estimations. This paper introduces **Uncertainty Quantification (UQ)**, a paradigm that formally models and propagates uncertainty through a system. We explore the critical distinction between aleatoric and epistemic uncertainty, present a practical implementation of an `UncertainValue` struct in Rust, and demonstrate its use in our `edge_simulation`, where it enables a sophisticated **probabilistic resource scheduler** that makes risk-aware decisions.

---

## 2. The Missing Fundamental: The Illusion of Certainty

Computer programs are often built on an illusion of certainty. An algorithm to calculate a task's cost is assumed to be precise. A sensor reading is treated as ground truth. This is a fragile model.
-   How long will a database query *really* take? The exact time depends on system load, caching, and other variables.
-   What is the power consumption of a CPU? It fluctuates with temperature and the specific instructions being executed.
-   A machine learning model's prediction is just that—a prediction, not a fact.

Systems that ignore this inherent uncertainty cannot make robust decisions. They cannot distinguish between a confident prediction and a wild guess. The missing fundamental is a framework for **representing, propagating, and acting on uncertainty**.

---

## 3. Foundational Research: Aleatoric vs. Epistemic Uncertainty

The field of AI safety provides a robust vocabulary for discussing uncertainty. Research in this area distinguishes between two primary types:
1.  **Aleatoric Uncertainty:** This refers to inherent, irreducible randomness in a system (e.g., the roll of a die). It represents statistical uncertainty.
2.  **Epistemic Uncertainty:** This refers to uncertainty caused by a lack of knowledge or an imperfect model (e.g., a machine learning model's prediction in an unfamiliar domain).

A truly robust system must be able to quantify both. While our implementation uses a simplified, unified model (a normal distribution), it is directly inspired by the probabilistic methods used in advanced UQ techniques. The goal, as described in recent AI safety research, is to build systems that "know what they don't know" and can act accordingly.

---

## 4. Rust Implementation: `UncertainValue`

Our `UncertainValue` struct provides a practical tool for bringing UQ into our system. It represents a value not as a single number, but as a **normal distribution defined by a mean and a standard deviation**.

### Core Mechanism

```rust
// from src/uncertainty_quantification.rs
#[derive(Debug, Clone, Copy)]
pub struct UncertainValue {
    pub mean: f64,
    pub std_dev: f64,
}
```

This simple structure allows us to perform two critical operations:
1.  **Propagation:** We can combine `UncertainValue`s. For example, the `add` method correctly calculates the new distribution for the sum of two uncertain values. This allows uncertainty to flow through a series of calculations.

    ```rust
    // from src/uncertainty_quantification.rs
    pub fn add(&self, other: &Self) -> Self {
        UncertainValue {
            mean: self.mean + other.mean,
            std_dev: (self.std_dev.powi(2) + other.std_dev.powi(2)).sqrt(),
        }
    }
    ```

2.  **Decision-Making:** We can query the probability of a value falling within a certain range using the `confidence` method, which is an implementation of the cumulative distribution function (CDF).

---

## 5. Verification and Demonstration

The power of this model is fully realized in the **probabilistic `ResourceAwareScheduler`** within our `edge_simulation`.

In the simulation, a `Task`'s computational cost is not a fixed number, but an `UncertainValue` (e.g., `1e9 ± 1e8` operations). The scheduler must decide whether to accept the task based on its resource budget.

Instead of a simple deterministic check, it uses the `confidence` method to calculate the **probability of a budget overrun**.

```rust
// from src/resource_aware.rs
let cpu_overload_prob = 1.0 - cost["cpu"].confidence(cpu_budget - self.consumed["cpu"]);
```

It only accepts the task if this probability is below a specified `risk_tolerance`. The simulation demonstrates this by submitting tasks with different risk tolerances, showing how the server can make more aggressive or conservative decisions based on the context.

This moves beyond simple "pass/fail" logic and allows the system to make nuanced, risk-aware decisions, which is the practical goal of Uncertainty Quantification.

---

### References
1.  Rudner, T. G. J., and Toner, H. 2024. *Key Concepts in AI Safety: Reliable Uncertainty Quantification in Machine Learning*. Center for Security and Emerging Technology.
2.  Abdar, M., et al. 2021. *A Review of Uncertainty Quantification in Deep Learning: Techniques, Applications and Challenges*. Information Fusion.
3.  *From Aleatoric to Epistemic: Exploring Uncertainty Quantification Techniques in Artificial Intelligence*. 2025. arXiv.
