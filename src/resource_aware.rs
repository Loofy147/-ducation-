use crate::uncertainty_quantification::UncertainValue;
use std::collections::HashMap;

/// Represents a computational task with various resource requirements.
pub struct Task {
    /// The name of the task.
    pub name: String,
    /// The number of operations required by the task.
    pub operations: UncertainValue,
    /// The amount of data the task needs to process.
    pub data_size: f64,
    /// Whether the task requires network access.
    pub network: bool,
    /// The value or priority of the task.
    pub value: f64,
}

/// Defines the resource budgets for the scheduler.
pub struct Budgets {
    /// The CPU budget.
    pub cpu: f64,
    /// The energy budget.
    pub energy: f64,
    /// The memory budget.
    pub memory: f64,
    /// The bandwidth budget.
    pub bandwidth: f64,
}

/// A scheduler that makes decisions based on resource availability and task requirements.
pub struct ResourceAwareScheduler {
    budgets: Budgets,
    consumed: HashMap<String, f64>,
}

impl ResourceAwareScheduler {
    /// Creates a new `ResourceAwareScheduler` with the given budgets.
    pub fn new(budgets: Budgets) -> Self {
        let mut consumed = HashMap::new();
        consumed.insert("cpu".to_string(), 0.0);
        consumed.insert("energy".to_string(), 0.0);
        consumed.insert("memory".to_string(), 0.0);
        consumed.insert("bandwidth".to_string(), 0.0);
        ResourceAwareScheduler { budgets, consumed }
    }

    fn estimate_cost(&self, task: &Task) -> HashMap<String, UncertainValue> {
        let mut cost = HashMap::new();
        cost.insert(
            "cpu".to_string(),
            UncertainValue::new(task.operations.mean / 1e9, task.operations.std_dev / 1e9),
        );
        cost.insert(
            "energy".to_string(),
            UncertainValue::new(task.operations.mean * 1e-9, task.operations.std_dev * 1e-9),
        );
        cost.insert(
            "memory".to_string(),
            UncertainValue::new(task.data_size, 0.0),
        );
        cost.insert(
            "bandwidth".to_string(),
            if task.network {
                UncertainValue::new(task.data_size, 0.0)
            } else {
                UncertainValue::new(0.0, 0.0)
            },
        );
        cost
    }

    /// Determines if a task can be scheduled without exceeding the resource budgets,
    /// given a certain risk tolerance.
    pub fn can_schedule(&self, task: &Task, risk_tolerance: f64) -> bool {
        let cost = self.estimate_cost(task);
        self.can_schedule_with_cost(&cost, risk_tolerance)
    }

    fn can_schedule_with_cost(
        &self,
        cost: &HashMap<String, UncertainValue>,
        risk_tolerance: f64,
    ) -> bool {
        let cpu_budget = self.budgets.cpu;
        let energy_budget = self.budgets.energy;
        let memory_budget = self.budgets.memory;
        let bandwidth_budget = self.budgets.bandwidth;

        let cpu_overload_prob = 1.0 - cost["cpu"].confidence(cpu_budget - self.consumed["cpu"]);
        let energy_overload_prob =
            1.0 - cost["energy"].confidence(energy_budget - self.consumed["energy"]);
        let memory_overload_prob =
            1.0 - cost["memory"].confidence(memory_budget - self.consumed["memory"]);
        let bandwidth_overload_prob =
            1.0 - cost["bandwidth"].confidence(bandwidth_budget - self.consumed["bandwidth"]);

        cpu_overload_prob < risk_tolerance
            && energy_overload_prob < risk_tolerance
            && memory_overload_prob < risk_tolerance
            && bandwidth_overload_prob < risk_tolerance
    }

    /// Schedules a task if it can be accommodated within the resource budgets.
    /// Returns `true` if the task was scheduled, `false` otherwise.
    pub fn schedule_task(&mut self, task: &Task, risk_tolerance: f64) -> bool {
        let cost = self.estimate_cost(task);
        if self.can_schedule_with_cost(&cost, risk_tolerance) {
            for (resource, value) in cost {
                *self.consumed.get_mut(&resource).unwrap() += value.mean; // Use the mean for accounting
            }
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_resource_optimization() {
        let budgets = Budgets {
            cpu: 10.0,
            energy: 100.0,
            memory: 1_000_000_000.0,
            bandwidth: 100_000_000.0,
        };
        let mut scheduler = ResourceAwareScheduler::new(budgets);

        let huge_task = Task {
            name: "Huge_Task".to_string(),
            operations: UncertainValue::new(1e12, 0.0), // Guaranteed to exceed budget
            data_size: 1e11,
            network: true,
            value: 100.0,
        };

        let rejected = !scheduler.schedule_task(&huge_task, 0.1);
        assert!(rejected, "The huge task should be rejected");
    }
}
