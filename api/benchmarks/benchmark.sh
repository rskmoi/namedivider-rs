#!/bin/bash

set -e

# Configuration
LATEST_PORT=8001
V030_PORT=8002
RESULTS_DIR="results"
TEST_DATA_DIR="test-data"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Create results directory
mkdir -p "$RESULTS_DIR"

# Function to print colored output
print_section() {
    echo -e "\n${BLUE}=== $1 ===${NC}\n"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

# Function to convert txt to JSON array
txt_to_json() {
    local txt_file="$1"
    local max_lines="$2"
    
    if [ -n "$max_lines" ]; then
        head -n "$max_lines" "$txt_file" | jq -R . | jq -s '{"names": .}'
    else
        cat "$txt_file" | jq -R . | jq -s '{"names": .}'
    fi
}

# Function to measure response time
measure_time() {
    local url="$1"
    local data="$2"
    local mode="$3"
    local iterations="${4:-5}"
    
    local total_time=0
    local success_count=0
    
    for i in $(seq 1 $iterations); do
        local start_time=$(date +%s.%N)
        local response=$(curl -s -X POST -H "Content-Type: application/json" -d "$data" "$url" 2>/dev/null)
        local end_time=$(date +%s.%N)
        
        if echo "$response" | jq -e '.divided_names' > /dev/null 2>&1; then
            local time_diff=$(echo "$end_time - $start_time" | bc)
            total_time=$(echo "$total_time + $time_diff" | bc)
            success_count=$((success_count + 1))
        fi
    done
    
    if [ "$success_count" -gt 0 ]; then
        echo "scale=4; $total_time / $success_count" | bc
    else
        echo "0"
    fi
}

# Function to start containers
start_containers() {
    print_section "Starting Docker Containers"
    
    # Stop existing containers
    docker stop namedivider-latest namedivider-v030 2>/dev/null || true
    
    # Start containers
    docker run -d --rm -p $LATEST_PORT:8000 --name namedivider-latest rskmoi/namedivider-api:latest
    docker run -d --rm -p $V030_PORT:8000 --name namedivider-v030 rskmoi/namedivider-api:0.3.0
    
    print_success "Containers started"
    sleep 5
}

# Function to stop containers
stop_containers() {
    print_section "Stopping Docker Containers"
    docker stop namedivider-latest namedivider-v030 2>/dev/null || true
    print_success "Containers stopped"
}

# Function to run benchmarks
run_benchmark() {
    local test_name="$1"
    local data_file="$2"
    local max_lines="$3"
    local mode="$4"
    
    print_section "Benchmark: $test_name"
    
    local json_data
    if [ -f "$data_file" ]; then
        json_data=$(txt_to_json "$data_file" "$max_lines")
    else
        print_error "Test data file not found: $data_file"
        return 1
    fi
    
    # Add mode if specified
    if [ -n "$mode" ]; then
        json_data=$(echo "$json_data" | jq --arg mode "$mode" '. + {mode: $mode}')
    fi
    
    local name_count=$(echo "$json_data" | jq '.names | length')
    echo "Testing with $name_count names"
    
    # Test latest version
    local latest_time=$(measure_time "localhost:$LATEST_PORT/divide" "$json_data" "$mode")
    local latest_per_name=$(echo "scale=4; $latest_time / $name_count" | bc)
    
    # Test v0.3.0
    local v030_time=$(measure_time "localhost:$V030_PORT/divide" "$json_data" "$mode")
    local v030_per_name=$(echo "scale=4; $v030_time / $name_count" | bc)
    
    # Calculate improvement
    local improvement_ratio=$(echo "scale=2; ($latest_time - $v030_time) / $latest_time * 100" | bc)
    
    # Output results
    echo "Results:"
    echo "  Latest (v0.1.0): ${latest_time}s total, ${latest_per_name}s per name"
    echo "  v0.3.0:          ${v030_time}s total, ${v030_per_name}s per name"
    echo "  Improvement:     ${improvement_ratio}%"
    
    # Save to results file
    local result_file="$RESULTS_DIR/benchmark_$(date +%Y%m%d_%H%M%S).txt"
    {
        echo "Benchmark: $test_name"
        echo "Date: $(date)"
        echo "Names: $name_count"
        echo "Mode: ${mode:-basic}"
        echo "Latest: ${latest_time}s (${latest_per_name}s per name)"
        echo "v0.3.0: ${v030_time}s (${v030_per_name}s per name)"
        echo "Improvement: ${improvement_ratio}%"
        echo "---"
    } >> "$result_file"
}

# Main execution
main() {
    print_section "NameDivider API Performance Benchmark"
    
    # Check dependencies
    if ! command -v jq &> /dev/null; then
        print_error "jq is required but not installed"
        exit 1
    fi
    
    if ! command -v bc &> /dev/null; then
        print_error "bc is required but not installed"
        exit 1
    fi
    
    start_containers
    
    # Test with different data sizes
    if [ -f "$TEST_DATA_DIR/10000names.txt" ]; then
        run_benchmark "Single name" "$TEST_DATA_DIR/10000names.txt" 1
        run_benchmark "Small batch (10 names)" "$TEST_DATA_DIR/10000names.txt" 10
        run_benchmark "Medium batch (100 names)" "$TEST_DATA_DIR/10000names.txt" 100
        run_benchmark "Large batch (1000 names)" "$TEST_DATA_DIR/10000names.txt" 1000
        
        # Test GBDT mode
        run_benchmark "Single name (GBDT)" "$TEST_DATA_DIR/10000names.txt" 1 "gbdt"
        run_benchmark "Small batch (10 names, GBDT)" "$TEST_DATA_DIR/10000names.txt" 10 "gbdt"
        run_benchmark "Medium batch (100 names, GBDT)" "$TEST_DATA_DIR/10000names.txt" 100 "gbdt"
    else
        print_error "10000names.txt not found. Please add your test data file."
        # Run with sample data
        run_benchmark "Sample data" "$TEST_DATA_DIR/sample.txt" 
    fi
    
    stop_containers
    
    print_success "Benchmark completed. Results saved in $RESULTS_DIR/"
}

# Handle script interruption
trap stop_containers EXIT

main "$@"