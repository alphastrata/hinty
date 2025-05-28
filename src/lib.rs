pub fn wikipedia_dataset_as_string() -> String {
    let data_path = "datasets/enwiki-latest-all-titles-in-ns0";

    if !std::path::Path::new(data_path).exists() {
        eprintln!("Error: Data file not found at {data_path}");
        eprintln!("Please download it from:");
        eprintln!("https://dumps.wikimedia.org/enwiki/latest/enwiki-latest-all-titles-in-ns0.gz");
        eprintln!("Extract it and place it in the datasets/ directory.");
        std::process::exit(1);
    }

    let data = match std::fs::read_to_string(data_path) {
        Ok(raw) => raw.to_lowercase().replace("_", " "),
        Err(e) => {
            eprintln!("Error reading file: {e}");
            std::process::exit(1);
        }
    };

    data
}

pub fn wikimedia_dataset_as_vec() -> Vec<String> {
    wikipedia_dataset_as_string()
        .split('\n')
        .map(|v| v.into())
        .collect()
}
