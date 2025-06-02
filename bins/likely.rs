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
            // NOTE: this is bad, it should be bad, most of the Titles in the wikipedia dataset
            // are NOT your search query.
            if std::intrinsics::likely(*title == q.to_lowercase()) {
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
