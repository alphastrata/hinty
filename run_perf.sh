#!/usr/bin/env bash

# perf_benchmark.sh
# Runs all test combinations with perf statistics
# Measures branch prediction, cache behavior, and CPU utilization

set -eo pipefail

# Configuration
FILES=(
    "90-10_likely.txt"
    "50-50.txt" 
    "10-90_unlikely.txt"
)

BINARIES=("baseline" "likely" "unlikely")
ITERATIONS=5          # Number of runs per test
WARMUP_RUNS=2         # Warmup runs excluded from measurements
RESULTS_DIR="perf_results"
LOG_PREFIX="$(date +%Y%m%d_%H%M%S)"  # Timestamp for unique results

# Ensure perf is available
if ! command -v perf &> /dev/null; then
    echo "Error: 'perf' tool not found. Please install linux-tools for your distribution."
    exit 1
fi

# Build release versions with debug symbols
echo -e "\033[1;34mBuilding release versions...\033[0m"
RUSTFLAGS="-g -C target-cpu=native" cargo build --release

# Create results directory
mkdir -p "${RESULTS_DIR}"

# Run all test combinations
for file in "${FILES[@]}"; do
    # Verify input file exists
    if [[ ! -f "$file" ]]; then
        echo -e "\033[1;31mError: Input file '$file' not found!\033[0m"
        continue
    fi

    for binary in "${BINARIES[@]}"; do
        # Verify binary exists
        if [[ ! -f "./target/release/$binary" ]]; then
            echo -e "\033[1;31mError: Binary '$binary' not found!\033[0m"
            continue
        fi

        echo -e "\n\033[1;32mTesting $file with $binary binary...\033[0m"
        
        # Warmup runs
        for ((i=1; i<=WARMUP_RUNS; i++)); do
            echo -ne "\rWarmup run $i/$WARMUP_RUNS"
            ./target/release/"$binary" "$file" > /dev/null
        done
        echo

        # Main measurement runs
        for ((i=1; i<=ITERATIONS; i++)); do
            echo -ne "\rMeasurement run $i/$ITERATIONS"
            
            # Run with perf and save to log file
            perf stat -e \
                branch-misses,branch-instructions,branches,\
                cache-misses,cache-references,\
                cycles,instructions,cpu-clock,task-clock \
                -o "${RESULTS_DIR}/${LOG_PREFIX}_${file%.*}_${binary}_run${i}.perf.log" \
                ./target/release/"$binary" "$file"
        done
        echo
    done
done

# Generate summary report
echo -e "\n\033[1;34mGenerating summary report...\033[0m"
{
    echo "Fibonacci Benchmark Results Summary"
    echo "================================="
    echo "Test Timestamp: ${LOG_PREFIX}"
    echo "Iterations: ${ITERATIONS} (after ${WARMUP_RUNS} warmup runs)"
    echo ""
    
    for file in "${FILES[@]}"; do
        echo "File: ${file}"
        echo "----------------"
        
        for binary in "${BINARIES[@]}"; do
            echo -e "\nBinary: ${binary}"
            
            # Aggregate all runs for this file/binary combination
            grep -E -h \
                "branch-misses|branch-instructions|branches|cache-misses|instructions per cycle" \
                "${RESULTS_DIR}/${LOG_PREFIX}_${file%.*}_${binary}"_*.perf.log | \
                awk '
                    BEGIN { 
                        printf("%-25s %12s %12s %12s\n", "Metric", "Min", "Avg", "Max") 
                        printf("--------------------------------------------------------\n")
                    }
                    /branch-misses/ { 
                        split($1, a, ","); 
                        gsub(/%/, "", a[1]); 
                        bm[NR]=a[1]; sum_bm+=a[1]; count++
                    }
                    /cache-misses/ {
                        split($1, a, ",");
                        gsub(/%/, "", a[1]);
                        cm[NR]=a[1]; sum_cm+=a[1]
                    }
                    /instructions per cycle/ {
                        ipc[NR]=$1; sum_ipc+=$1
                    }
                    END {
                        asort(bm); asort(cm); asort(ipc);
                        printf("%-25s %12.2f %12.2f %12.2f\n", "Branch Miss %", bm[1], sum_bm/count, bm[count]);
                        printf("%-25s %12.2f %12.2f %12.2f\n", "Cache Miss %", cm[1], sum_cm/count, cm[count]);
                        printf("%-25s %12.2f %12.2f %12.2f\n", "IPC", ipc[1], sum_ipc/count, ipc[count]);
                    }
                '
        done
    done
} > "${RESULTS_DIR}/${LOG_PREFIX}_summary_report.txt"

echo -e "\n\033[1;32mBenchmark completed!\033[0m"
echo -e "Results saved in: \033[1m${RESULTS_DIR}/\033[0m"
echo -e "Summary report: \033[1m${RESULTS_DIR}/${LOG_PREFIX}_summary_report.txt\033[0m"