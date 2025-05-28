#!/usr/bin/env bash

# perf_benchmark.sh
# Runs all test combinations with perf statistics
# so we can see branch misses and the % thereof

set -e

# Input files
FILES=(
    "90-10_likely.txt"
    "50-50.txt"
    "10-90_unlikely.txt"
)

BINARIES=("baseline" "likely" "unlikely")

echo "Building release versions..."
RUSTFLAGS="-g" cargo build --release

RESULTS_DIR="perf_results"
mkdir -p "$RESULTS_DIR"

# Run all combinations
for file in "${FILES[@]}"; do
    for binary in "${BINARIES[@]}"; do
        echo -e "\nRunning $file with $binary binary..."
        
        # Run with perf and save to log file
        perf stat -e branch-misses,branch-instructions,branches,cache-misses,cache-references \
            ./target/release/$binary \
            "$file" \
            2> "${RESULTS_DIR}/${file%.*}_${binary}.perf.log"
        
        # Add human readable header to log
        echo -e "\n=== $file - $binary ===" | cat - "${RESULTS_DIR}/${file%.*}_${binary}.perf.log" > temp && mv temp "${RESULTS_DIR}/${file%.*}_${binary}.perf.log"
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
        
        for binary in "${BINARIES[@]}"; do
            logfile="${RESULTS_DIR}/${file%.*}_${binary}.perf.log"
            
            echo -e "\nBinary: $binary"
            grep -E "branch-misses|branch-instructions|branches|cache-misses" "$logfile" | grep -v "seconds time elapsed"
        done
        
        echo ""
    done
} > "${RESULTS_DIR}/summary_report.txt"

echo -e "\nBenchmark completed!"
echo "Results saved in: $RESULTS_DIR/"
echo "Summary report: ${RESULTS_DIR}/summary_report.txt"