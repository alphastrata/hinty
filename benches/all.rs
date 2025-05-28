#![feature(likely_unlikely)]

use criterion::{Criterion, criterion_group, criterion_main};
use hinty::NumberSet;
use std::{
    collections::HashSet,
    hint::{black_box, likely, unlikely},
    path::PathBuf,
};

fn bench_is_fib(c: &mut Criterion) {
    let files = [
        ("90-10", "90-10_likely.txt"),
        ("50-50", "50-50.txt"),
        ("10-90", "10-90_unlikely.txt"),
    ];

    for (name, path) in files {
        let number_set = NumberSet::from_files(&[PathBuf::from(path)]).unwrap();
        let numbers = number_set.numbers();
        let answers = &number_set.fib_numbers;

        let mut group = c.benchmark_group(format!("is_fib_{}", name));

        group.bench_function("standard", |b| {
            b.iter(|| {
                #[inline(never)]
                fn is_fib(fib_numbers: &HashSet<u64>, n: u64) -> bool {
                    fib_numbers.contains(&n)
                }
                for &n in numbers.iter() {
                    black_box(is_fib(answers, n));
                }
            })
        });

        group.bench_function("likely", |b| {
            b.iter(|| {
                #[inline(never)]
                fn is_fib(fib_numbers: &HashSet<u64>, n: u64) -> bool {
                    likely(fib_numbers.contains(&n))
                }

                for &n in numbers.iter() {
                    black_box(is_fib(answers, n));
                }
            })
        });

        group.bench_function("unlikely", |b| {
            b.iter(|| {
                #[inline(never)]
                fn is_fib(fib_numbers: &HashSet<u64>, n: u64) -> bool {
                    unlikely(fib_numbers.contains(&n))
                }

                for &n in numbers.iter() {
                    black_box(is_fib(answers, n));
                }
            })
        });

        group.finish();
    }
}

criterion_group!(benches, bench_is_fib);
criterion_main!(benches);
