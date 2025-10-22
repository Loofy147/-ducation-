use std::collections::HashMap;

#[derive(Clone)]
pub struct TreatmentData {
    pub treated: bool,
    pub outcome: bool,
    pub confounding_variable: String,
}

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

pub fn analyze_data_by_group(data: &[TreatmentData]) -> HashMap<String, (f64, f64)> {
    let mut grouped_data: HashMap<String, Vec<TreatmentData>> = HashMap::new();
    for d in data {
        grouped_data
            .entry(d.confounding_variable.clone())
            .or_insert_with(Vec::new)
            .push(d.clone());
    }

    let mut results = HashMap::new();
    for (group, group_data) in grouped_data {
        let (treated_rate, untreated_rate) = analyze_data(&group_data);
        results.insert(group, (treated_rate, untreated_rate));
    }
    results
}
