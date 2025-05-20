use criterion::{Criterion, black_box, criterion_group, criterion_main};
use hinty::NumberSet;
use std::path::PathBuf;

fn bench_is_fib(c: &mut Criterion) {
    let files = [
        ("90-10", "90-10_likely.txt"),
        ("50-50", "50-50.txt"),
        ("10-90", "10-90_unlikely.txt"),
    ];

    for (name, path) in files {
        let number_set = NumberSet::from_files(&[PathBuf::from(path)]).unwrap();
        let numbers = number_set.numbers();

        let mut group = c.benchmark_group(format!("is_fib_{}", name));

        group.bench_function("standard", |b| {
            b.iter(|| {
                for &n in numbers.iter() {
                    black_box(number_set.is_fib(n));
                }
            })
        });

        group.bench_function("likely", |b| {
            b.iter(|| {
                for &n in numbers.iter() {
                    black_box(number_set.is_fib_likely(n));
                }
            })
        });

        group.bench_function("unlikely", |b| {
            b.iter(|| {
                for &n in numbers.iter() {
                    black_box(number_set.is_fib_unlikely(n));
                }
            })
        });

        group.finish();
    }
}

criterion_group!(benches, bench_is_fib);
criterion_main!(benches);
