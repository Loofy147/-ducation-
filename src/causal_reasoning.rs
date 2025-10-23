use std::collections::HashMap;

/// Represents a single data point for causal analysis.
#[derive(Clone)]
pub struct TreatmentData {
    /// Whether the treatment was applied.
    pub treated: bool,
    /// The outcome of the experiment.
    pub outcome: bool,
    /// A confounding variable that might affect the outcome.
    pub confounding_variable: String,
}

/// Analyzes the overall success rates for treated and untreated groups.
pub fn analyze_data(data: &[TreatmentData]) -> (f64, f64) {
    let mut treated_success = 0;
    let mut treated_total = 0;
    let mut untreated_success = 0;
    let mut untreated_total = 0;

    for d in data {
        if d.treated {
            treated_total += 1;
            if d.outcome {
                treated_success += 1;
            }
        } else {
            untreated_total += 1;
            if d.outcome {
                untreated_success += 1;
            }
        }
    }

    (
        treated_success as f64 / treated_total as f64,
        untreated_success as f64 / untreated_total as f64,
    )
}

/// Analyzes the success rates for treated and untreated groups, stratified by a confounding variable.
pub fn analyze_data_by_group(data: &[TreatmentData]) -> HashMap<String, (f64, f64)> {
    let mut grouped_data: HashMap<String, Vec<TreatmentData>> = HashMap::new();
    for d in data {
        grouped_data
            .entry(d.confounding_variable.clone())
            .or_default()
            .push(d.clone());
    }

    let mut results = HashMap::new();
    for (group, group_data) in grouped_data {
        let (treated_rate, untreated_rate) = analyze_data(&group_data);
        results.insert(group, (treated_rate, untreated_rate));
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, SeedableRng};
    use rand::rngs::StdRng;

    #[test]
    fn test_verify_causal_reasoning() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut data = Vec::new();
        // Group "Easy": Legacy used more. Optimized is better (95% vs 90%).
        for _ in 0..20 { data.push(TreatmentData { treated: true, outcome: rng.gen_bool(0.95), confounding_variable: "Easy".to_string() }); }
        for _ in 0..80 { data.push(TreatmentData { treated: false, outcome: rng.gen_bool(0.90), confounding_variable: "Easy".to_string() }); }

        // Group "Hard": Optimized used more. Optimized is better (30% vs 20%).
        for _ in 0..80 { data.push(TreatmentData { treated: true, outcome: rng.gen_bool(0.30), confounding_variable: "Hard".to_string() }); }
        for _ in 0..20 { data.push(TreatmentData { treated: false, outcome: rng.gen_bool(0.20), confounding_variable: "Hard".to_string() }); }

        let (overall_optimized, overall_legacy) = analyze_data(&data);
        let by_group = analyze_data_by_group(&data);

        let paradox = overall_optimized < overall_legacy;
        let correct_conclusion = by_group.values().all(|(t, u)| t > u);

        assert!(paradox, "Simpson's Paradox should be observed");
        assert!(correct_conclusion, "The correct conclusion should be drawn from the grouped data");
    }
}
