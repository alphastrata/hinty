use clap::Parser;
use hinty::NumberSet;
use std::{collections::HashSet, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input files to process
    #[arg(short, long, required = true)]
    input: Vec<PathBuf>,
}

#[inline(never)]
fn is_fib(fib_numbers: &HashSet<u64>, n: u64) -> bool {
    fib_numbers.contains(&n)
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let number_set = NumberSet::from_files(&args.input)?;
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
