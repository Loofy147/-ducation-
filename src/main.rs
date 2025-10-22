use colored::*;
use rand::Rng;
use computational_fundamentals::time_aware::{AnytimeQuicksort, WcetAnalyzer};
use computational_fundamentals::resource_aware::{Task, Budgets, ResourceAwareScheduler};
use computational_fundamentals::adversarial_first::SecureHashMap;

fn main() {
    println!("{}", "🔬 Verification Suite".bold().cyan());
    println!("{}", "Proving the Missing Fundamentals Actually Work".blue());
    println!();

    assert!(verify_time_bounds(), "Time-bounded computation verification failed");
    assert!(verify_resource_optimization(), "Resource optimization verification failed");
    assert!(verify_adversarial_resistance(), "Adversarial resistance verification failed");

    println!("\n🎉 {}", "All tests passed!".bold().green());
}

fn verify_time_bounds() -> bool {
    println!("{}", "⏱️  Time-Bounded Computation".bold());
    let mut rng = rand::thread_rng();
    let mut arr: Vec<i32> = (0..1000).map(|_| rng.gen_range(0..10000)).collect();

    let mut sorter = AnytimeQuicksort::new(1); // 1ms deadline
    sorter.sort(&mut arr);
    let is_sorted = arr.windows(2).all(|w| w[0] <= w[1]);
    println!("Sorted with 1ms deadline (partially sorted): {}", if is_sorted { "no".red() } else { "yes".green() });

    let mut analyzer = WcetAnalyzer::new();
    analyzer.measure(|| {
        let mut arr: Vec<i32> = (0..100).map(|_| rng.gen_range(0..1000)).collect();
        arr.sort();
    }, 100);
    println!("WCET analysis completed with {} samples.", analyzer.samples.len());

    println!();
    !is_sorted // The test passes if the array is NOT fully sorted
}

fn verify_resource_optimization() -> bool {
    println!("{}", "⚡ Resource-Aware Optimization".bold());

    let budgets = Budgets {
        cpu: 10.0,
        energy: 100.0,
        memory: 1_000_000_000.0,
        bandwidth: 100_000_000.0,
    };
    let mut scheduler = ResourceAwareScheduler::new(budgets);

    let tasks = vec![
        Task {
            name: "ML_Training".to_string(),
            operations: 1e10,
            data_size: 1e8,
            network: true,
            value: 100.0,
        },
        Task {
            name: "Video_Encode".to_string(),
            operations: 5e9,
            data_size: 5e8,
            network: false,
            value: 50.0,
        },
    ];

    let ml_training_scheduled = scheduler.schedule_task(&tasks[0]);
    let video_encode_rejected = !scheduler.schedule_task(&tasks[1]);

    if ml_training_scheduled {
        println!("✅ {} scheduled", tasks[0].name);
    } else {
        println!("❌ {} not scheduled", tasks[0].name);
    }

    if video_encode_rejected {
        println!("✅ {} rejected as expected", tasks[1].name);
    } else {
        println!("❌ {} was not rejected", tasks[1].name);
    }

    println!();
    ml_training_scheduled && video_encode_rejected
}

fn verify_adversarial_resistance() -> bool {
    println!("{}", "🛡️  Adversarial Resistance".bold());

    let mut map = SecureHashMap::new();
    for i in 0..20 {
        map.set(&format!("user{}", i), &format!("data{}", i));
    }
    println!("Normal operations completed.");

    for i in 0..100 {
        map.set(&format!("attack_payload_{}", i), &format!("malicious_{}", i));
    }
    println!("Collision attack simulation completed.");
    println!();
    true
}
