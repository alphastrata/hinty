use clap::Parser;
use rand::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of numbers to generate
    #[arg(short, long)]
    n: u64,

    /// Percentage of numbers that should be Fibonacci numbers (0-100)
    #[arg(short, long)]
    p: Option<u8>,

    /// Output string
    #[arg(short, long)]
    o: String,
}

fn generate_fib_numbers() -> (Vec<u64>, HashSet<u64>) {
    let mut fib_list = Vec::new();
    let mut fib_set = HashSet::new();
    let (mut a, mut b) = (0u64, 1u64);

    // Add initial numbers and handle duplicates
    for num in [a, b] {
        fib_list.push(num);
        fib_set.insert(num);
    }

    while let Some(next) = a.checked_add(b) {
        fib_list.push(next);
        fib_set.insert(next);
        a = b;
        b = next;
    }

    (fib_list, fib_set)
}

fn generate_and_save_numbers(n: u64, percentage: Option<u8>, output: &str) -> std::io::Result<()> {
    let file = File::create(output)?;
    let mut writer = BufWriter::new(file);
    let mut rng = rand::rng();

    let (fib_list, fib_set) = generate_fib_numbers();

    match percentage {
        Some(percent) => {
            let percent = percent.clamp(0, 100);
            let fib_count = ((n as f64 * percent as f64) / 100.0).round() as u64;
            let non_fib_count = n - fib_count;

            let mut numbers = Vec::with_capacity(n as usize);

            // Generate Fibonacci numbers
            for _ in 0..fib_count {
                numbers.push(fib_list[rng.random_range(0..fib_list.len())]);
            }

            // Generate non-Fibonacci numbers
            for _ in 0..non_fib_count {
                loop {
                    let num = rng.random();
                    if !fib_set.contains(&num) {
                        numbers.push(num);
                        break;
                    }
                }
            }

            // Shuffle and write
            numbers.shuffle(&mut rng);
            for num in numbers {
                writeln!(writer, "{num}")?;
            }
        }
        None => {
            // Generate all random numbers
            for _ in 0..n {
                let num: u64 = rng.random();
                writeln!(writer, "{num}")?;
            }
        }
    }

    writer.flush()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    generate_and_save_numbers(args.n, args.p, &args.o)
}
