use anyhow::Result;
use colored::*;
use rand::Rng;
use computational_fundamentals::{
    time_aware::AnytimeQuicksort,
    resource_aware::{Budgets, ResourceAwareScheduler, Task},
    adversarial_first::SecureHashMap,
    uncertainty_quantification::UncertainValue,
    self_modifying::SelfOptimizingCache,
    algebraic_composability::{TaskStats, task_stats_monoid},
    causal_reasoning::{TreatmentData, analyze_data, analyze_data_by_group},
};

struct EdgeServer {
    scheduler: ResourceAwareScheduler,
    cache: SelfOptimizingCache<String, String>,
    stats: TaskStats,
    causal_data: Vec<TreatmentData>,
}

impl EdgeServer {
    fn new(budgets: Budgets) -> Self {
        EdgeServer {
            scheduler: ResourceAwareScheduler::new(budgets),
            cache: SelfOptimizingCache::new(10),
            stats: task_stats_monoid().identity(),
            causal_data: Vec::new(),
        }
    }

    fn process_task(&mut self, task: &Task, risk_tolerance: f64, use_optimized: Option<&str>) {
        if let Some(result) = self.cache.get(&task.name) {
            println!("⚡ Cache hit for task '{}'.", task.name);
            self.stats = (task_stats_monoid().operation)(self.stats.clone(), TaskStats {
                tasks_processed: 1,
                data_processed: 0.0,
            });
            return;
        }

        if self.scheduler.schedule_task(task, risk_tolerance) {
            println!("✅ Task '{}' accepted.", task.name);
            let outcome = if let Some(task_type) = use_optimized {
                let mut rng = rand::thread_rng();
                let success_rate = match (task_type == "Optimized", task.value > 50.0) {
                    (true, false) => 0.95, // Optimized, Easy
                    (false, false) => 0.90, // Legacy, Easy
                    (true, true) => 0.30,  // Optimized, Hard
                    (false, true) => 0.20, // Legacy, Hard
                };
                rng.gen_bool(success_rate)
            } else {
                true // Non-causal tasks are always successful if processed
            };

            if let Some(task_type) = use_optimized {
                 self.causal_data.push(TreatmentData {
                    treated: task_type == "Optimized",
                    outcome,
                    confounding_variable: if task.value > 50.0 { "Hard".to_string() } else { "Easy".to_string() },
                });
            }

            self.stats = (task_stats_monoid().operation)(self.stats.clone(), TaskStats {
                tasks_processed: 1,
                data_processed: task.data_size,
            });

            self.cache.put(task.name.clone(), format!("result_{}", outcome));
            println!("   Task '{}' processed (Success: {}).", task.name, outcome);
        } else {
            println!("❌ Task '{}' rejected.", task.name);
        }
    }
}

fn main() -> Result<()> {
    println!("{}", "🚀 Starting Full Edge Server Simulation".bold().yellow());
    let budgets = Budgets { cpu: 1000.0, energy: 10000.0, memory: 1e11, bandwidth: 1e10 };
    let mut server = EdgeServer::new(budgets);

    println!("\n{}", "-- Phase 1: LRU-favoring workload (Scanning) --".bold());
    for i in 0..101 {
        let task = Task { name: format!("scan_{}", i), operations: UncertainValue::new(1e8, 1e7), data_size: 1e7, network: false, value: 1.0 };
        server.process_task(&task, 0.1, None);
    }

    println!("\n{}", "-- Phase 2: LFU-favoring workload (Repeated Access) --".bold());
    for _ in 0..101 {
        let task = Task { name: "repeated_hotspot_task".to_string(), operations: UncertainValue::new(1e8, 1e7), data_size: 1e7, network: false, value: 1.0 };
        server.process_task(&task, 0.1, None);
    }

    println!("\n{}", "-- Phase 3: Causal Analysis Workload --".bold());
    // Easy tasks
    for i in 0..20 {
        let task = Task { name: format!("easy_opt_{}", i), operations: UncertainValue::new(1e8, 1e7), data_size: 1e7, network: false, value: 1.0 };
        server.process_task(&task, 0.1, Some("Optimized"));
    }
    for i in 0..80 {
        let task = Task { name: format!("easy_leg_{}", i), operations: UncertainValue::new(1e8, 1e7), data_size: 1e7, network: false, value: 1.0 };
        server.process_task(&task, 0.1, Some("Legacy"));
    }
    // Hard tasks
    for i in 0..80 {
        let task = Task { name: format!("hard_opt_{}", i), operations: UncertainValue::new(1e9, 1e8), data_size: 1e8, network: true, value: 100.0 };
        server.process_task(&task, 0.1, Some("Optimized"));
    }
    for i in 0..20 {
        let task = Task { name: format!("hard_leg_{}", i), operations: UncertainValue::new(1e9, 1e8), data_size: 1e8, network: true, value: 100.0 };
        server.process_task(&task, 0.1, Some("Legacy"));
    }

    println!("\n{}", "-- Final Analysis --".bold());
    println!("Final Stats: {:?}", server.stats);
    let (overall_optimized, overall_legacy) = analyze_data(&server.causal_data);
    println!("Causal Analysis (Overall): Optimized={:.2}, Legacy={:.2}", overall_optimized, overall_legacy);
    let by_group = analyze_data_by_group(&server.causal_data);
    for (group, (optimized, legacy)) in by_group {
        println!("  Causal Analysis (Group {}): Optimized={:.2}, Legacy={:.2}", group, optimized, legacy);
    }

    println!("\n{}", "✅ Simulation Complete".bold().green());
    Ok(())
}
