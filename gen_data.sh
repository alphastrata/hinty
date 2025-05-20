#!/usr/bin/env bash

# build_datasets.sh
# Generates datasets with the same naming conventions I used in the blog post.

set -e

# Configuration
NUMBERS=$1 # how many numbers per file?
OUTPUT_DIR="."

# Build release version if needed
if [[ ! -f "./target/release/generate" ]]; then
    echo "Building generator..."
    cargo build --release --bin generate
fi

# Generate datasets
echo "Generating test datasets (10 million numbers each)..."

# 90% Fibonacci (90-10_likely.txt)
echo "Creating 90-10 split (90% Fibonacci)..."
./target/release/generate \
    -n $NUMBERS \
    -p 90 \
    -o "$OUTPUT_DIR/90-10_likely.txt"

# 50% Fibonacci (50-50.txt)
echo "Creating 50-50 split..."
./target/release/generate \
    -n $NUMBERS \
    -p 50 \
    -o "$OUTPUT_DIR/50-50.txt"

# 10% Fibonacci (10-90_unlikely.txt)
echo "Creating 10-90 split (10% Fibonacci)..."
./target/release/generate \
    -n $NUMBERS \
    -p 10 \
    -o "$OUTPUT_DIR/10-90_unlikely.txt"

# Verify file sizes
echo -e "\nGenerated files:"
ls -lh "$OUTPUT_DIR"/*

echo -e "\nDataset generation complete!"
echo "Files saved in: $OUTPUT_DIR/"
