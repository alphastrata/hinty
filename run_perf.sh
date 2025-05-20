#!/usr/bin/env bash

# perf_benchmark.sh
# Runs all test combinations with perf statistics
# so we can see branch misses and the % thereof

set -e

# Input files, these will be different for you if you named them differently to how I did...
FILES=(
    "90-10_likely.txt"
    "50-50.txt"
    "10-90_unlikely.txt"
)

VARIANTS=("standard" "likely" "unlikely")

echo "Building release version..."
RUSTFLAGS="-g" cargo build --release --bin run

RESULTS_DIR="perf_results"
mkdir -p "$RESULTS_DIR"

# Run all combinations
for file in "${FILES[@]}"; do
    for variant in "${VARIANTS[@]}"; do
        echo -e "\nRunning $file with $variant variant..."
        
        # Run with perf and save to log file
        perf stat -e branch-misses,branch-instructions,branches,cache-misses,cache-references \
            ./target/release/run \
            --input "$file" \
            --variant "$variant" \
            2> "${RESULTS_DIR}/${file%.*}_${variant}.perf.log"
        
        # Add human readable header to log
        echo -e "\n=== $file - $variant ===" | cat - "${RESULTS_DIR}/${file%.*}_${variant}.perf.log" > temp && mv temp "${RESULTS_DIR}/${file%.*}_${variant}.perf.log"
    done
done

# Generate summary report
echo -e "\nGenerating summary report..."
{
    echo "Fibonacci Benchmark Results Summary"
    echo "================================="
    echo ""
    
    for file in "${FILES[@]}"; do
        echo "File: $file"
        echo "----------------"
        
        for variant in "${VARIANTS[@]}"; do
            logfile="${RESULTS_DIR}/${file%.*}_${variant}.perf.log"
            
            echo -e "\nVariant: $variant"
            grep -E "branch-misses|branch-instructions|branches|cache-misses" "$logfile" | grep -v "seconds time elapsed"
        done
        
        echo ""
    done
} > "${RESULTS_DIR}/summary_report.txt"

echo -e "\nBenchmark completed!"
echo "Results saved in: $RESULTS_DIR/"
echo "Summary report: ${RESULTS_DIR}/summary_report.txt"
