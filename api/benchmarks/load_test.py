#!/usr/bin/env python3
"""
NameDivider API Load Test Script

Multiple processes send random requests with different modes:
- basic mode
- gbdt mode  
- no mode specified (defaults to basic)

Tests concurrent processing capability and correctness.
"""

import json
import random
import time
import argparse
import sys
import subprocess
import signal
import atexit
from concurrent.futures import ProcessPoolExecutor, as_completed
from dataclasses import dataclass
from typing import List, Optional
import requests
import threading

@dataclass
class TestResult:
    """Test result data structure"""
    process_id: int
    request_id: int
    mode: Optional[str]
    success: bool
    response_time: float
    error_message: Optional[str] = None
    response_data: Optional[dict] = None

class LoadTester:
    def __init__(self, base_url: str, test_names: List[str]):
        self.base_url = base_url
        self.test_names = test_names
        self.modes = [None, "basic", "gbdt"]  # None means no mode specified
        
    def send_request(self, process_id: int, request_id: int, batch_size: int) -> TestResult:
        """Send a single request to the API"""
        # Randomly select mode and names
        mode = random.choice(self.modes)
        selected_names = random.sample(self.test_names, min(batch_size, len(self.test_names)))
        
        # Prepare request data
        request_data = {"names": selected_names}
        if mode is not None:
            request_data["mode"] = mode
            
        start_time = time.time()
        try:
            response = requests.post(
                f"{self.base_url}/divide",
                json=request_data,
                headers={"Content-Type": "application/json"},
                timeout=30
            )
            response_time = time.time() - start_time
            
            if response.status_code == 200:
                response_data = response.json()
                # Validate response structure
                if "divided_names" in response_data and isinstance(response_data["divided_names"], list):
                    if len(response_data["divided_names"]) == len(selected_names):
                        return TestResult(
                            process_id=process_id,
                            request_id=request_id,
                            mode=mode,
                            success=True,
                            response_time=response_time,
                            response_data=response_data
                        )
                    else:
                        return TestResult(
                            process_id=process_id,
                            request_id=request_id,
                            mode=mode,
                            success=False,
                            response_time=response_time,
                            error_message=f"Response count mismatch: expected {len(selected_names)}, got {len(response_data['divided_names'])}"
                        )
                else:
                    return TestResult(
                        process_id=process_id,
                        request_id=request_id,
                        mode=mode,
                        success=False,
                        response_time=response_time,
                        error_message="Invalid response structure"
                    )
            else:
                return TestResult(
                    process_id=process_id,
                    request_id=request_id,
                    mode=mode,
                    success=False,
                    response_time=response_time,
                    error_message=f"HTTP {response.status_code}: {response.text}"
                )
                
        except requests.exceptions.RequestException as e:
            response_time = time.time() - start_time
            return TestResult(
                process_id=process_id,
                request_id=request_id,
                mode=mode,
                success=False,
                response_time=response_time,
                error_message=str(e)
            )

def worker_process(args):
    """Worker process function for multiprocessing"""
    process_id, num_requests, base_url, test_names, batch_size = args
    
    tester = LoadTester(base_url, test_names)
    results = []
    
    for request_id in range(num_requests):
        result = tester.send_request(process_id, request_id, batch_size)
        results.append(result)
        
        # Add small random delay to avoid synchronized requests
        time.sleep(random.uniform(0.01, 0.1))
    
    return results

def load_test_names(file_path: str) -> List[str]:
    """Load test names from file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            names = [line.strip() for line in f if line.strip()]
        print(f"Loaded {len(names)} test names from {file_path}")
        return names
    except FileNotFoundError:
        print(f"Test data file not found: {file_path}")
        # Return some default test names
        return [
            "竈門炭治郎", "竈門禰豆子", "我妻善逸", "嘴平伊之助",
            "冨岡義勇", "胡蝶しのぶ", "煉獄杏寿郎", "宇髄天元",
            "時透無一郎", "甘露寺蜜璃", "伊黒小芭内", "不死川実弥",
            "悲鳴嶼行冥", "産屋敷耀哉", "鱗滝左近次", "桑島慈悟郎"
        ]

def print_statistics(all_results: List[TestResult]):
    """Print test statistics"""
    print("\n" + "="*60)
    print("LOAD TEST RESULTS")
    print("="*60)
    
    total_requests = len(all_results)
    successful_requests = sum(1 for r in all_results if r.success)
    failed_requests = total_requests - successful_requests
    
    print(f"Total Requests: {total_requests}")
    print(f"Successful: {successful_requests} ({successful_requests/total_requests*100:.1f}%)")
    print(f"Failed: {failed_requests} ({failed_requests/total_requests*100:.1f}%)")
    
    if successful_requests > 0:
        successful_results = [r for r in all_results if r.success]
        response_times = [r.response_time for r in successful_results]
        
        print(f"\nResponse Time Statistics:")
        print(f"  Average: {sum(response_times)/len(response_times):.3f}s")
        print(f"  Min: {min(response_times):.3f}s")
        print(f"  Max: {max(response_times):.3f}s")
        
        # Statistics by mode
        print(f"\nResults by Mode:")
        for mode in [None, "basic", "gbdt"]:
            mode_results = [r for r in successful_results if r.mode == mode]
            if mode_results:
                mode_name = "default" if mode is None else mode
                avg_time = sum(r.response_time for r in mode_results) / len(mode_results)
                print(f"  {mode_name}: {len(mode_results)} requests, avg {avg_time:.3f}s")
    
    if failed_requests > 0:
        print(f"\nError Summary:")
        failed_results = [r for r in all_results if not r.success]
        error_counts = {}
        for result in failed_results:
            error = result.error_message or "Unknown error"
            error_counts[error] = error_counts.get(error, 0) + 1
        
        for error, count in error_counts.items():
            print(f"  {error}: {count} times")

class DockerManager:
    """Manages Docker container lifecycle for testing"""
    def __init__(self, image_tag: str = "0.3.0", port: int = 8000, container_name: str = "namedivider-load-test"):
        self.image_tag = image_tag
        self.port = port
        self.container_name = container_name
        self.container_id = None
        
    def start_container(self):
        """Start the NameDivider API Docker container"""
        print(f"\nStarting Docker container: rskmoi/namedivider-api:{self.image_tag}")
        
        # Stop existing container if running
        try:
            subprocess.run(["docker", "stop", self.container_name], 
                         capture_output=True, check=False)
        except Exception:
            pass
        
        # Start new container
        try:
            result = subprocess.run([
                "docker", "run", "-d", "--rm", 
                "-p", f"{self.port}:8000",
                "--name", self.container_name,
                f"rskmoi/namedivider-api:{self.image_tag}"
            ], capture_output=True, text=True, check=True)
            
            self.container_id = result.stdout.strip()
            print(f"✓ Container started: {self.container_id[:12]}")
            
            # Wait for API to be ready
            print("Waiting for API to be ready...")
            max_retries = 30
            for i in range(max_retries):
                try:
                    response = requests.get(f"http://localhost:{self.port}/health", timeout=2)
                    if response.status_code == 200:
                        print(f"✓ API is ready (took {i+1} attempts)")
                        return True
                except requests.exceptions.RequestException:
                    pass
                time.sleep(1)
            
            print("✗ API did not become ready within 30 seconds")
            return False
            
        except subprocess.CalledProcessError as e:
            print(f"✗ Failed to start container: {e.stderr}")
            return False
    
    def stop_container(self):
        """Stop the Docker container"""
        if self.container_name:
            print(f"\nStopping Docker container: {self.container_name}")
            try:
                subprocess.run(["docker", "stop", self.container_name], 
                             capture_output=True, check=False)
                print("✓ Container stopped")
            except Exception as e:
                print(f"✗ Error stopping container: {e}")

def main():
    parser = argparse.ArgumentParser(description="NameDivider API Load Test")
    parser.add_argument("--url", default="http://localhost:8000", 
                       help="API base URL (default: http://localhost:8000)")
    parser.add_argument("--processes", type=int, default=4,
                       help="Number of processes (default: 4)")
    parser.add_argument("--requests-per-process", type=int, default=10,
                       help="Number of requests per process (default: 10)")
    parser.add_argument("--batch-size", type=int, default=5,
                       help="Number of names per request (default: 5)")
    parser.add_argument("--test-data", default="test-data/10000names.txt",
                       help="Test data file path (default: test-data/10000names.txt)")
    parser.add_argument("--no-docker", action="store_true",
                       help="Skip Docker container management (use existing API)")
    parser.add_argument("--image-tag", default="0.3.0",
                       help="Docker image tag (default: 0.3.0)")
    
    args = parser.parse_args()
    
    print("NameDivider API Load Test")
    print(f"URL: {args.url}")
    print(f"Processes: {args.processes}")
    print(f"Requests per process: {args.requests_per_process}")
    print(f"Batch size: {args.batch_size}")
    print(f"Total requests: {args.processes * args.requests_per_process}")
    print(f"Docker management: {'disabled' if args.no_docker else 'enabled'}")
    if not args.no_docker:
        print(f"Docker image: rskmoi/namedivider-api:{args.image_tag}")
    
    # Docker container management
    docker_manager = None
    if not args.no_docker:
        docker_manager = DockerManager(image_tag=args.image_tag)
        
        # Register cleanup function
        def cleanup():
            if docker_manager:
                docker_manager.stop_container()
        atexit.register(cleanup)
        
        # Handle interrupt signals
        def signal_handler(signum, frame):
            print("\nReceived interrupt signal, cleaning up...")
            cleanup()
            sys.exit(1)
        signal.signal(signal.SIGINT, signal_handler)
        signal.signal(signal.SIGTERM, signal_handler)
        
        # Start container
        if not docker_manager.start_container():
            print("Failed to start Docker container")
            sys.exit(1)
    
    # Load test names
    test_names = load_test_names(args.test_data)
    if len(test_names) < args.batch_size:
        print(f"Warning: Only {len(test_names)} test names available, but batch size is {args.batch_size}")
        args.batch_size = len(test_names)
    
    # Test API connectivity
    try:
        response = requests.get(f"{args.url}/health", timeout=5)
        print(f"API health check: {response.status_code}")
    except requests.exceptions.RequestException as e:
        print(f"Error: Could not connect to API: {e}")
        if docker_manager:
            docker_manager.stop_container()
        sys.exit(1)
    
    print("\nStarting load test...")
    start_time = time.time()
    
    # Prepare arguments for worker processes
    worker_args = [
        (i, args.requests_per_process, args.url, test_names, args.batch_size)
        for i in range(args.processes)
    ]
    
    all_results = []
    
    # Execute load test with multiprocessing
    with ProcessPoolExecutor(max_workers=args.processes) as executor:
        future_to_process = {
            executor.submit(worker_process, arg): arg[0] 
            for arg in worker_args
        }
        
        for future in as_completed(future_to_process):
            process_id = future_to_process[future]
            try:
                results = future.result()
                all_results.extend(results)
                print(f"Process {process_id} completed: {len(results)} requests")
            except Exception as e:
                print(f"Process {process_id} failed: {e}")
    
    total_time = time.time() - start_time
    print(f"\nLoad test completed in {total_time:.2f} seconds")
    
    # Print statistics
    print_statistics(all_results)
    
    # Save detailed results to file
    timestamp = time.strftime("%Y%m%d_%H%M%S")
    results_file = f"results/load_test_{timestamp}.json"
    
    try:
        with open(results_file, 'w', encoding='utf-8') as f:
            json.dump([
                {
                    'process_id': r.process_id,
                    'request_id': r.request_id,
                    'mode': r.mode,
                    'success': r.success,
                    'response_time': r.response_time,
                    'error_message': r.error_message
                }
                for r in all_results
            ], f, indent=2, ensure_ascii=False)
        print(f"\nDetailed results saved to: {results_file}")
    except Exception as e:
        print(f"Could not save results file: {e}")
    
    # Cleanup
    if docker_manager:
        docker_manager.stop_container()
    
    # Exit with error code if there were failures
    failed_count = sum(1 for r in all_results if not r.success)
    if failed_count > 0:
        print(f"\nWARNING: {failed_count} requests failed")
        sys.exit(1)
    else:
        print("\nAll requests completed successfully!")
        sys.exit(0)

if __name__ == "__main__":
    main()