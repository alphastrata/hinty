#![feature(likely_unlikely)]
use std::collections::HashSet;
use std::env;
use std::hint::likely;
use std::path::PathBuf;

use hinty::NumberSet;

#[inline(never)]
fn is_fib(fib_numbers: &HashSet<u64>, n: u64) -> bool {
    fib_numbers.contains(&n)
}

fn main() -> std::io::Result<()> {
    let paths = env::args()
        .skip(1)
        .map(PathBuf::from)
        .collect::<Vec<PathBuf>>();

    let number_set = NumberSet::from_files(&paths)?;
    let answers = &number_set.fib_numbers;
    // NOTE I'd normally write code like this commented out section,
    // HOWEVER according to the docs: https://doc.rust-lang.org/std/intrinsics/fn.likely.html
    // unless you use an `if` statement hints are likely to have NO effect.
    // let fib_count = number_set
    //     .numbers()
    //     .iter()
    //     .filter(|n| is_fib(answers, **n))
    //     .count();
    let mut fib_count = 0;
    for n in number_set.numbers().iter() {
        if likely(is_fib(answers, *n)) {
            fib_count += 1;
        }
        // No `else` needed here since we only count matches
    }

    let total = number_set.numbers().len();
    println!("Total numbers: {total}");
    println!(
        "Fibonacci numbers: {} ({:.2}%)",
        fib_count,
        (fib_count as f64 / total as f64) * 100.0
    );

    Ok(())
}
