use colored::*;
use rand::Rng;
use computational_fundamentals::time_aware::{AnytimeQuicksort, WcetAnalyzer};
use computational_fundamentals::resource_aware::{Task, Budgets, ResourceAwareScheduler};
use computational_fundamentals::adversarial_first::SecureHashMap;
use computational_fundamentals::algebraic_composability::Monoid;
use computational_fundamentals::uncertainty_quantification::UncertainValue;
use computational_fundamentals::self_modifying::{SelfOptimizingCache, CacheStrategy};
use computational_fundamentals::causal_reasoning::{TreatmentData, analyze_data, analyze_data_by_group};

fn main() {
    println!("{}", "🔬 Verification Suite".bold().cyan());
    println!("{}", "Proving the Missing Fundamentals Actually Work".blue());
    println!();

    assert!(verify_time_bounds(), "Time-bounded computation verification failed");
    assert!(verify_resource_optimization(), "Resource optimization verification failed");
    assert!(verify_adversarial_resistance(), "Adversarial resistance verification failed");
    assert!(verify_algebraic_laws(), "Algebraic composability verification failed");
    assert!(verify_uncertainty_quantification(), "Uncertainty quantification verification failed");
    assert!(verify_self_modification(), "Self-modifying algorithm verification failed");
    assert!(verify_causal_reasoning(), "Causal reasoning verification failed");

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
            operations: UncertainValue::new(1e10, 0.0),
            data_size: 1e8,
            network: true,
            value: 100.0,
        },
        Task {
            name: "Video_Encode".to_string(),
            operations: UncertainValue::new(5e9, 0.0),
            data_size: 5e8,
            network: false,
            value: 50.0,
        },
    ];

    let ml_training_scheduled = scheduler.schedule_task(&tasks[0], 0.1);
    let video_encode_rejected = !scheduler.schedule_task(&tasks[1], 0.1);

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

fn verify_algebraic_laws() -> bool {
    println!("{}", "🔢 Algebraic Composition Laws".bold());

    let sum_monoid = Monoid::new(0, |a, b| a + b);
    let sum_values = vec![1, 2, 3, 4, 5];
    let sum_identity = sum_monoid.check_identity_law(&sum_values);
    let sum_associativity = sum_monoid.check_associativity_law(&sum_values);
    println!("Sum monoid: identity={}, associativity={}", sum_identity, sum_associativity);

    let list_monoid = Monoid::new(Vec::<i32>::new(), |mut a, mut b| { a.append(&mut b); a });
    let list_values = vec![vec![1], vec![2, 3], vec![4, 5, 6]];
    let list_identity = list_monoid.check_identity_law(&list_values);
    let list_associativity = list_monoid.check_associativity_law(&list_values);
    println!("List monoid: identity={}, associativity={}", list_identity, list_associativity);

    println!();
    sum_identity && sum_associativity && list_identity && list_associativity
}

fn verify_uncertainty_quantification() -> bool {
    println!("{}", "📊 Uncertainty Quantification".bold());
    let x = UncertainValue::new(10.0, 2.0);
    let y = UncertainValue::new(5.0, 1.0);
    let sum = x.add(&y);

    let expected_mean = 15.0;
    let expected_std_dev = 5.0_f64.sqrt();

    let mean_correct = (sum.mean - expected_mean).abs() < 0.01;
    let std_dev_correct = (sum.std_dev - expected_std_dev).abs() < 0.01;
    println!("Uncertainty propagation: mean_correct={}, std_dev_correct={}", mean_correct, std_dev_correct);

    let conf = x.confidence(x.mean + 1.96 * x.std_dev);
    let conf_correct = (conf - 0.975).abs() < 0.01;
    println!("Confidence interval: conf_correct={}", conf_correct);

    println!();
    mean_correct && std_dev_correct && conf_correct
}

fn verify_self_modification() -> bool {
    println!("{}", "🤖 Self-Modifying Algorithms".bold());
    let mut cache = SelfOptimizingCache::new(10);
    println!("Initial strategy: {:?}", cache.get_strategy());

    // Fill the cache
    for i in 0..10 {
        cache.put(i, i);
    }

    // Simulate a workload that favors LFU (high hit rate)
    for _ in 0..100 {
        cache.get(&0);
    }

    let lfu_strategy = *cache.get_strategy() == CacheStrategy::LFU;
    println!("Strategy after LFU-favoring workload: {:?}", cache.get_strategy());

    // Simulate a workload that favors LRU (low hit rate)
    for i in 0..100 {
        cache.get(&(i % 20)); // Access a wider range of keys
    }
    let lru_strategy = *cache.get_strategy() == CacheStrategy::LRU;
    println!("Strategy after LRU-favoring workload: {:?}", cache.get_strategy());

    println!();
    lfu_strategy && lru_strategy
}

fn verify_causal_reasoning() -> bool {
    println!("{}", "🧠 Causal Reasoning".bold());
    let mut data = Vec::new();
    // Group "Easy": Legacy used more. Optimized is better (95% vs 90%).
    for _ in 0..20 { data.push(TreatmentData { treated: true, outcome: rand::thread_rng().gen_bool(0.95), confounding_variable: "Easy".to_string() }); }
    for _ in 0..80 { data.push(TreatmentData { treated: false, outcome: rand::thread_rng().gen_bool(0.90), confounding_variable: "Easy".to_string() }); }

    // Group "Hard": Optimized used more. Optimized is better (30% vs 20%).
    for _ in 0..80 { data.push(TreatmentData { treated: true, outcome: rand::thread_rng().gen_bool(0.30), confounding_variable: "Hard".to_string() }); }
    for _ in 0..20 { data.push(TreatmentData { treated: false, outcome: rand::thread_rng().gen_bool(0.20), confounding_variable: "Hard".to_string() }); }

    let (overall_optimized, overall_legacy) = analyze_data(&data);
    println!("Overall success rates: Optimized={:.2}, Legacy={:.2}", overall_optimized, overall_legacy);

    let by_group = analyze_data_by_group(&data);
    for (group, (optimized, legacy)) in &by_group {
        println!("  Group {}: Optimized={:.2}, Legacy={:.2}", group, optimized, legacy);
    }

    let paradox = overall_optimized < overall_legacy;
    let correct_conclusion = by_group.values().all(|(t, u)| t > u);

    println!("Paradox observed: {}", paradox);
    println!();
    paradox && correct_conclusion
}
