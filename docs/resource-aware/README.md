# Paper 2: Resource-Aware Computing

**A Framework for Multi-Objective Optimization in Constrained Systems**

**Version:** 1.0
**Status:** Complete

---

## 1. Abstract

Classical algorithms are designed to optimize for a single resource: CPU cycles. Production systems, however, must operate within a complex web of constraints, including energy, memory, and network bandwidth. This paper introduces **Resource-Aware Computing**, a paradigm that treats system resources as a multi-dimensional optimization problem. We explore the theory behind **multi-objective scheduling**, present a practical implementation of a `ResourceAwareScheduler` in Rust, and demonstrate its effectiveness in our `edge_simulation`, where it performs probabilistic admission control to prevent server overload.

---

## 2. The Missing Fundamental: Beyond a Single Objective

The traditional goal of algorithmic optimization is to minimize computation time. This uni-dimensional view is insufficient for modern systems:
-   A mobile device must balance performance with **battery life**.
-   An IoT sensor has strict limits on **memory and power**.
-   A cloud service must manage **CPU, memory, and network bandwidth** to serve thousands of users without exceeding its budget.

Focusing on just one resource inevitably leads to the over-consumption of another. The missing fundamental is a computational model that can reason about and optimize for **multiple, often competing, resource constraints simultaneously**.

---

## 3. Foundational Research: Multi-Objective Scheduling

Resource-aware computing is a well-established field in operating systems and distributed computing. Modern research, particularly in edge computing, frames this as a **multi-objective optimization problem**.

As highlighted in recent studies on edge networks, the goal is to balance competing objectives like **latency and energy consumption**. A decision that improves one (e.g., running the CPU faster to reduce latency) can negatively impact the other (increasing energy use). This requires a scheduler that can navigate these trade-offs intelligently.

Advanced approaches model this using formalisms like **Multi-Objective Markov Decision Processes (MOMDPs)** and apply reinforcement learning to find optimal scheduling policies. While our implementation is simpler, it is based on the same foundational principles of multi-objective admission control.

---

## 4. Rust Implementation: `ResourceAwareScheduler`

Our `ResourceAwareScheduler` provides a concrete implementation of these concepts. It is initialized with a `Budgets` struct that defines the server's total capacity for multiple resources (CPU, energy, memory, etc.).

### Core Mechanism: Probabilistic Admission Control

The key innovation in our implementation is the integration of **Uncertainty Quantification**. In a real system, the resource cost of a task is an *estimate*, not a certainty. Our `Task` struct reflects this by defining its `operations` cost as an `UncertainValue` (a mean and a standard deviation).

When a task arrives, the scheduler's `schedule_task` method performs **probabilistic admission control**:
1.  It estimates the uncertain cost of the task for each resource.
2.  It calculates the probability that accepting the task will cause the server to exceed its budget for *any* of the resources.
3.  It compares this probability to a given `risk_tolerance`. The task is only accepted if the risk of overload is below this threshold.

```rust
// from src/resource_aware.rs
pub fn schedule_task(&mut self, task: &Task, risk_tolerance: f64) -> bool {
    // ... calculate overload probabilities ...

    cpu_overload_prob < risk_tolerance &&
    energy_overload_prob < risk_tolerance &&
    // ... etc.
}
```

This approach is far more robust than a deterministic scheduler. A low `risk_tolerance` (e.g., 1%) makes the scheduler conservative, which is ideal for mission-critical systems. A higher tolerance (e.g., 20%) makes it more aggressive, which might be suitable for non-critical, high-throughput applications.

---

## 5. Verification and Demonstration

The power of this approach is demonstrated in our `edge_simulation`. The simulation includes two phases:
1.  **Normal Operations:** The server processes a series of tasks with a **low risk tolerance (5%)**. It accepts tasks as long as it can be highly confident they won't cause an overload.
2.  **High-Risk Task:** A very expensive task arrives. With the low tolerance, it would be rejected. However, the simulation submits it with a **high risk tolerance (50%)**, modeling a scenario where a high-priority task is allowed to take a bigger risk.

The simulation output shows the scheduler correctly accepting tasks in the first phase and then making a risk-adjusted decision on the high-priority task. This demonstrates a production-ready approach to resource management that is both flexible and mathematically sound.

---

### References

1.  *Resource Scheduling Algorithm for Edge Computing Networks Based on Multi-Objective Optimization*. 2024. Applied Sciences.
2.  Barrett, C., and Narayanan, S. 2008. *An Introduction to Multi-Objective Optimization for Engineering Design*. Springer.
