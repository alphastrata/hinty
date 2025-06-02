#![allow(internal_features)]
#![feature(core_intrinsics)]
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use hinty::wikimedia_dataset_as_vec;

fn benchmark_search(c: &mut Criterion) {
    // Load the data once for all benchmarks
    let data = wikimedia_dataset_as_vec();
    
    let test_query = "rust programming language".to_string();

    let mut group = c.benchmark_group("std::hint::likely/unlikely/None");
    group.sample_size(100);
    

    // BASELINE
    group.bench_function("baseline", |b| {
        b.iter(|| {
            let mut misses = 0;
            for title in data.iter() {
                if *title == black_box(&test_query).to_lowercase() {
                    break;
                } else {
                    misses += 1;
                }
            }
            black_box(misses);
        })
    });


    // SHOULD BE THE BEST
    group.bench_function("unlikely", |b| {
        b.iter(|| {
            let mut misses = 0;
            for title in data.iter() {
                if std::intrinsics::unlikely(*title == black_box(&test_query).to_lowercase()) {
                    break;
                } else {
                    misses += 1;
                }
            }
            black_box(misses);
        })
    });

    // SHOULD BE WORSE
    group.bench_function("likely", |b| {
        b.iter(|| {
            let mut misses = 0;
            for title in data.iter() {
                if std::intrinsics::likely(*title == black_box(&test_query).to_lowercase()) {
                    break;
                } else {
                    misses += 1;
                }
            }
            black_box(misses);
        })
    });
}

criterion_group!(benches, benchmark_search);
criterion_main!(benches);