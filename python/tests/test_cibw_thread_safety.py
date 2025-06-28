#!/usr/bin/env python3
"""
Simple thread safety test for cibuildwheel test environment
"""

import namedivider_core
import concurrent.futures
import time
import sys

def main():
    print("=== Thread Safety Test inside build environment ===")
    
    # Import test
    print("✓ namedivider-core imported successfully")
    
    # Single thread test
    print("=== Single Thread Test ===")
    divider = namedivider_core.GBDTNameDivider()
    result = divider.divide_name("菅義偉")
    print(f"Single thread: {result.family} | {result.given}")
    
    # Multi thread test
    print("=== Multi Thread Test ===")
    
    def worker(worker_id):
        for i in range(50):
            result = divider.divide_name("田中太郎")
            time.sleep(0.001)
        return worker_id
    
    try:
        with concurrent.futures.ThreadPoolExecutor(max_workers=8) as executor:
            futures = [executor.submit(worker, i) for i in range(8)]
            for future in concurrent.futures.as_completed(futures, timeout=30):
                worker_id = future.result()
                print(f"Worker {worker_id} completed")
        print("✅ Multi-thread test completed successfully")
        return 0
    except Exception as e:
        print(f"❌ Multi-thread test failed: {e}")
        return 1

if __name__ == "__main__":
    sys.exit(main())