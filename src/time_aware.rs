use std::time::{Duration, Instant};

pub struct AnytimeQuicksort {
    deadline: Duration,
    start_time: Option<Instant>,
}

impl AnytimeQuicksort {
    pub fn new(deadline_ms: u64) -> Self {
        AnytimeQuicksort {
            deadline: Duration::from_millis(deadline_ms),
            start_time: None,
        }
    }

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

pub struct WcetAnalyzer {
    pub samples: Vec<f64>,
}

impl WcetAnalyzer {
    pub fn new() -> Self {
        WcetAnalyzer {
            samples: Vec::new(),
        }
    }

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
