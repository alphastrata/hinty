use clap::Parser;
use hinty::NumberSet;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input files to process
    #[arg(short, long, required = true)]
    input: Vec<PathBuf>,

    /// Which version of the check to use
    #[arg(short, long, value_enum, default_value_t = CheckVariant::Standard)]
    variant: CheckVariant,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum CheckVariant {
    /// Standard check (no hint)
    Standard,
    /// Optimized for likely Fibonacci numbers
    Likely,
    /// Optimized for unlikely Fibonacci numbers
    Unlikely,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let number_set = NumberSet::from_files(&args.input)?;

    let fib_count = std::hint::black_box(
        number_set
            .numbers()
            .iter()
            .filter(|&&n| match args.variant {
                CheckVariant::Standard => number_set.is_fib(n),
                CheckVariant::Likely => number_set.is_fib_likely(n),
                CheckVariant::Unlikely => number_set.is_fib_unlikely(n),
            })
            .count(),
    );

    let total = number_set.numbers().len();
    println!("Using {:?} check:", args.variant);
    println!("Total numbers: {}", total);
    println!(
        "Fibonacci numbers: {} ({:.2}%)",
        fib_count,
        (fib_count as f64 / total as f64) * 100.0
    );

    Ok(())
}
