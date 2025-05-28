#![feature(likely_unlikely)]
use std::collections::HashSet;
use std::env;
use std::hint::likely;
use std::path::PathBuf;

use hinty::NumberSet;

#[inline(never)]
fn is_fib(fib_numbers: &HashSet<u64>, n: u64) -> bool {
    likely(fib_numbers.contains(&n))
}

fn main() -> std::io::Result<()> {
    let paths = env::args()
        .skip(1)
        .map(|v| PathBuf::from(v))
        .collect::<Vec<PathBuf>>();

    let number_set = NumberSet::from_files(&paths)?;
    let answers = &number_set.fib_numbers;
    let fib_count = std::hint::black_box(
        number_set
            .numbers()
            .iter()
            .filter(|n| is_fib(&answers, **n))
            .count(),
    );

    let total = number_set.numbers().len();
    println!("Total numbers: {}", total);
    println!(
        "Fibonacci numbers: {} ({:.2}%)",
        fib_count,
        (fib_count as f64 / total as f64) * 100.0
    );

    Ok(())
}
