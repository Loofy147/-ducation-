use crate::uncertainty_quantification::UncertainValue;
use std::collections::HashMap;

pub struct Task {
    pub name: String,
    pub operations: UncertainValue,
    pub data_size: f64,
    pub network: bool,
    pub value: f64,
}

pub struct Budgets {
    pub cpu: f64,
    pub energy: f64,
    pub memory: f64,
    pub bandwidth: f64,
}

pub struct ResourceAwareScheduler {
    budgets: Budgets,
    consumed: HashMap<String, f64>,
}

impl ResourceAwareScheduler {
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
