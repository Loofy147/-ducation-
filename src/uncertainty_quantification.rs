use statrs::function::erf::erf;

/// A struct representing a value with uncertainty, described by a mean and a standard deviation.
#[derive(Debug, Clone, Copy)]
pub struct UncertainValue {
    /// The mean of the value.
    pub mean: f64,
    /// The standard deviation of the value.
    pub std_dev: f64,
}

impl UncertainValue {
    /// Creates a new `UncertainValue`.
    pub fn new(mean: f64, std_dev: f64) -> Self {
        UncertainValue { mean, std_dev }
    }

    /// Adds two `UncertainValue`s.
    pub fn add(&self, other: &Self) -> Self {
        UncertainValue {
            mean: self.mean + other.mean,
            std_dev: (self.std_dev.powi(2) + other.std_dev.powi(2)).sqrt(),
        }
    }

    /// Calculates the confidence (CDF) of a given value.
    pub fn confidence(&self, value: f64) -> f64 {
        0.5 * (1.0 + erf((value - self.mean) / (self.std_dev * 2.0_f64.sqrt())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_uncertainty_quantification() {
        let x = UncertainValue::new(10.0, 2.0);
        let y = UncertainValue::new(5.0, 1.0);
        let sum = x.add(&y);

        let expected_mean = 15.0;
        let expected_std_dev = 5.0_f64.sqrt();

        let mean_correct = (sum.mean - expected_mean).abs() < 0.01;
        let std_dev_correct = (sum.std_dev - expected_std_dev).abs() < 0.01;
        assert!(mean_correct, "The mean of the sum should be correct");
        assert!(std_dev_correct, "The standard deviation of the sum should be correct");

        let conf = x.confidence(x.mean + 1.96 * x.std_dev);
        let conf_correct = (conf - 0.975).abs() < 0.01;
        assert!(conf_correct, "The confidence interval should be correct");
    }
}
