#!/usr/bin/env bash

set -eo pipefail

# Configuration
FILES=("90-10_likely.txt" "50-50.txt" "10-90_unlikely.txt")
BINARIES=("baseline" "likely" "unlikely")
RESULTS_DIR="hyperfine_results"
mkdir -p "$RESULTS_DIR"

# Build binaries
echo "Building release binaries..."
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Run benchmarks
for file in "${FILES[@]}"; do
  for binary in "${BINARIES[@]}"; do
    echo "Benchmarking $binary with $file..."
    
    hyperfine \
      --warmup 5 \
      --runs 100 \
      --export-json "${RESULTS_DIR}/${binary}_${file%.*}.json" \
      "./target/release/$binary $file"
  done
done

# Generate summary CSV
echo "Generating summary..."
echo "file,binary,mean_time,stddev" > results_summary.csv
for file in "${FILES[@]}"; do
  for binary in "${BINARIES[@]}"; do
    mean=$(jq -r '.results[0].mean' "${RESULTS_DIR}/${binary}_${file%.*}.json")
    stddev=$(jq -r '.results[0].stddev' "${RESULTS_DIR}/${binary}_${file%.*}.json")
    echo "$file,$binary,$mean,$stddev" >> results_summary.csv
  done
done

echo "Benchmark complete!"
echo "Raw data: $RESULTS_DIR/"
echo "Summary: results_summary.csv"