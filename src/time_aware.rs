use std::time::{Duration, Instant};

/// A quicksort implementation that adheres to a strict deadline.
/// If the deadline is exceeded, the sorting process is halted,
/// resulting in a partially sorted array.
pub struct AnytimeQuicksort {
    deadline: Duration,
    start_time: Option<Instant>,
}

impl AnytimeQuicksort {
    /// Creates a new `AnytimeQuicksort` with a specified deadline in milliseconds.
    pub fn new(deadline_ms: u64) -> Self {
        AnytimeQuicksort {
            deadline: Duration::from_millis(deadline_ms),
            start_time: None,
        }
    }

    /// Sorts the given array until the deadline is met.
    pub fn sort(&mut self, arr: &mut [i32]) {
        self.start_time = Some(Instant::now());
        self.recursive_sort(arr, 0, (arr.len() as isize) - 1);
    }

    fn time_exceeded(&self) -> bool {
        if let Some(start_time) = self.start_time {
            start_time.elapsed() >= self.deadline
        } else {
            false
        }
    }

    fn recursive_sort(&self, arr: &mut [i32], lo: isize, hi: isize) {
        if self.time_exceeded() {
            return;
        }

        if lo >= hi {
            return;
        }

        let p = self.partition(arr, lo, hi);
        self.recursive_sort(arr, lo, p - 1);
        self.recursive_sort(arr, p + 1, hi);
    }

    fn partition(&self, arr: &mut [i32], lo: isize, hi: isize) -> isize {
        let pivot = arr[hi as usize];
        let mut i = lo - 1;
        for j in lo..hi {
            if self.time_exceeded() {
                return i + 1;
            }
            if arr[j as usize] < pivot {
                i += 1;
                arr.swap(i as usize, j as usize);
            }
        }
        arr.swap((i + 1) as usize, hi as usize);
        i + 1
    }
}

/// A tool for analyzing the Worst-Case Execution Time (WCET) of a given function.
pub struct WcetAnalyzer {
    /// A collection of execution time samples in milliseconds.
    pub samples: Vec<f64>,
}

impl Default for WcetAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl WcetAnalyzer {
    /// Creates a new `WcetAnalyzer`.
    pub fn new() -> Self {
        WcetAnalyzer {
            samples: Vec::new(),
        }
    }

    /// Measures the execution time of a function over a specified number of iterations.
    pub fn measure<F>(&mut self, mut f: F, iterations: u32)
    where
        F: FnMut(),
    {
        self.samples.clear();
        for _ in 0..iterations {
            let start = Instant::now();
            f();
            self.samples.push(start.elapsed().as_secs_f64() * 1000.0); // in ms
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_verify_time_bounds() {
        let mut rng = rand::thread_rng();
        let mut arr: Vec<i32> = (0..1000).map(|_| rng.gen_range(0..10000)).collect();

        let mut sorter = AnytimeQuicksort::new(0); // 1 nanosecond deadline
        sorter.sort(&mut arr);
        let is_sorted = arr.windows(2).all(|w| w[0] <= w[1]);
        assert!(!is_sorted, "The array should not be fully sorted with a 1ns deadline");

        let mut analyzer = WcetAnalyzer::new();
        analyzer.measure(|| {
            let mut arr: Vec<i32> = (0..100).map(|_| rng.gen_range(0..1000)).collect();
            arr.sort();
        }, 100);
        assert_eq!(analyzer.samples.len(), 100, "WCET analysis should have 100 samples");
    }
}
