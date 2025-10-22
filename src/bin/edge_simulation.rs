use anyhow::Result;
use colored::*;
use rand::Rng;
use computational_fundamentals::{
    time_aware::AnytimeQuicksort,
    resource_aware::{Budgets, ResourceAwareScheduler, Task},
    adversarial_first::SecureHashMap,
};

struct EdgeServer {
    scheduler: ResourceAwareScheduler,
    task_metadata: SecureHashMap,
}

impl EdgeServer {
    fn new(budgets: Budgets) -> Self {
        EdgeServer {
            scheduler: ResourceAwareScheduler::new(budgets),
            task_metadata: SecureHashMap::new(),
        }
    }

    fn process_task(&mut self, task: &Task, deadline_ms: u64) {
        if self.scheduler.schedule_task(task) {
            println!("✅ Task '{}' accepted.", task.name);
            self.task_metadata.set(&task.name, "accepted");
            let mut data: Vec<i32> = (0..1000).map(|_| rand::thread_rng().gen()).collect();
            let mut sorter = AnytimeQuicksort::new(deadline_ms);
            sorter.sort(&mut data);
            println!("   Task '{}' processed with deadline {}ms.", task.name, deadline_ms);
        } else {
            println!("❌ Task '{}' rejected due to resource constraints.", task.name);
        }
    }
}

fn main() -> Result<()> {
    println!("{}", "🚀 Starting Edge Server Simulation".bold().yellow());

    let budgets = Budgets {
        cpu: 100.0,
        energy: 1000.0,
        memory: 10_000_000_000.0,
        bandwidth: 1_000_000_000.0,
    };
    let mut server = EdgeServer::new(budgets);

    println!("\n{}", "-- Phase 1: Normal Operations --".bold());
    for i in 0..10 {
        let task = Task {
            name: format!("task_{}", i),
            operations: 1e9,
            data_size: 1e8,
            network: true,
            value: 10.0,
        };
        server.process_task(&task, 10);
    }

    println!("\n{}", "-- Phase 2: High-Priority Task --".bold());
    let high_priority_task = Task {
        name: "high_priority_task".to_string(),
        operations: 5e9,
        data_size: 5e8,
        network: true,
        value: 100.0,
    };
    server.process_task(&high_priority_task, 5);

    println!("\n{}", "-- Phase 3: Adversarial Attack --".bold());
    println!("Finding colliding keys...");
    let mut colliding_keys = Vec::new();
    let target_hash = server.task_metadata.hash("find_me");
    for i in 0..100000 {
        let key = format!("colliding_key_{}", i);
        if server.task_metadata.hash(&key) == target_hash {
            colliding_keys.push(key);
            if colliding_keys.len() >= 50 {
                break;
            }
        }
    }
    println!("Found {} colliding keys. Launching attack...", colliding_keys.len());

    for key in colliding_keys {
        let task = Task {
            name: key,
            operations: 1e8,
            data_size: 1e7,
            network: false,
            value: 1.0,
        };
        server.process_task(&task, 20);
    }

    println!("\n{}", "✅ Simulation Complete".bold().green());
    Ok(())
}
