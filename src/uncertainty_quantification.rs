use statrs::function::erf::erf;

/// A struct representing a value with uncertainty, described by a mean and a standard deviation.
#[derive(Debug, Clone, Copy)]
pub struct UncertainValue {
    pub mean: f64,
    pub std_dev: f64,
}

impl UncertainValue {
    /// Creates a new UncertainValue.
    pub fn new(mean: f64, std_dev: f64) -> Self {
        UncertainValue { mean, std_dev }
    }

    /// Adds two UncertainValues.
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
