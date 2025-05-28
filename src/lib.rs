use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub struct NumberSet {
    pub numbers: Vec<u64>,
    pub fib_numbers: HashSet<u64>,
}

impl NumberSet {
    /// Load numbers from multiple files and precompute Fibonacci numbers in range
    pub fn from_files<P: AsRef<Path>>(paths: &[P]) -> io::Result<Self> {
        let mut numbers = Vec::new();
        let mut max_num = 0u64;

        // Read and parse numbers from all files
        for path in paths {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            for line in reader.lines() {
                let num: u64 = line?.parse().map_err(|e: std::num::ParseIntError| {
                    io::Error::new(io::ErrorKind::InvalidData, e.to_string())
                })?;
                max_num = max_num.max(num);
                numbers.push(num);
            }
        }

        // Precompute Fibonacci numbers up to the maximum found
        let fib_numbers = Self::generate_fib_numbers_up_to(max_num);

        Ok(Self {
            numbers,
            fib_numbers,
        })
    }

    /// Generate all Fibonacci numbers up to a given maximum
    fn generate_fib_numbers_up_to(max: u64) -> HashSet<u64> {
        let mut fib_numbers = HashSet::new();
        let (mut a, mut b) = (0u64, 1u64);

        fib_numbers.insert(a);
        fib_numbers.insert(b);

        while let Some(next) = a.checked_add(b) {
            if next > max {
                break;
            }
            fib_numbers.insert(next);
            a = b;
            b = next;
        }

        fib_numbers
    }

    /// Get reference to loaded numbers
    pub fn numbers(&self) -> &[u64] {
        &self.numbers
    }

    /// Get count of Fibonacci numbers in loaded set
    pub fn fib_count(&self) -> usize {
        self.numbers
            .iter()
            .filter(|&&n| self.fib_numbers.contains(&n))
            .count()
    }

    /// Get the percentage of Fibonacci numbers in loaded set
    pub fn fib_percentage(&self) -> f64 {
        if self.numbers.is_empty() {
            0.0
        } else {
            self.fib_count() as f64 / self.numbers.len() as f64 * 100.0
        }
    }
}
