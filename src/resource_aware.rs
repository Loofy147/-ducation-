use std::collections::HashMap;

pub struct Task {
    pub name: String,
    pub operations: f64,
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

    fn estimate_cost(&self, task: &Task) -> HashMap<String, f64> {
        let mut cost = HashMap::new();
        cost.insert("cpu".to_string(), task.operations / 1e9);
        cost.insert("energy".to_string(), task.operations * 1e-9);
        cost.insert("memory".to_string(), task.data_size);
        cost.insert(
            "bandwidth".to_string(),
            if task.network { task.data_size } else { 0.0 },
        );
        cost
    }

    pub fn can_schedule(&self, task: &Task) -> bool {
        let cost = self.estimate_cost(task);
        self.can_schedule_with_cost(&cost)
    }

    fn can_schedule_with_cost(&self, cost: &HashMap<String, f64>) -> bool {
        let cpu_budget = self.budgets.cpu;
        let energy_budget = self.budgets.energy;
        let memory_budget = self.budgets.memory;
        let bandwidth_budget = self.budgets.bandwidth;

        cost.get("cpu").map_or(true, |&c| self.consumed["cpu"] + c <= cpu_budget) &&
        cost.get("energy").map_or(true, |&c| self.consumed["energy"] + c <= energy_budget) &&
        cost.get("memory").map_or(true, |&c| self.consumed["memory"] + c <= memory_budget) &&
        cost.get("bandwidth").map_or(true, |&c| self.consumed["bandwidth"] + c <= bandwidth_budget)
    }

    pub fn schedule_task(&mut self, task: &Task) -> bool {
        let cost = self.estimate_cost(task);
        if self.can_schedule_with_cost(&cost) {
            for (resource, value) in cost {
                *self.consumed.get_mut(&resource).unwrap() += value;
            }
            true
        } else {
            false
        }
    }
}
