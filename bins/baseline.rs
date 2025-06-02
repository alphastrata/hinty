#![allow(internal_features)]
#![feature(core_intrinsics)]
use hinty::wikimedia_dataset_as_vec;
use std::env;

fn main() {
    let queries = env::args().collect::<Vec<String>>();

    let data = wikimedia_dataset_as_vec();
    let mut misses = 0;

    println!("Entries: {}", data.len());

    for q in queries.iter().skip(1) {
        println!("Searching... {q}");
        
        for title in data.iter() {
            if *title == q.to_lowercase() {
                println!("{q} was located");
                break;
            }
            else {
                misses += 1;
            }
        }
    }

    println!("Misses : {misses}");
}
