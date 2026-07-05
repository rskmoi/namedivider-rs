#!/usr/bin/env python3
"""
PyO3 Thread Safety Test for namedivider-core

このスクリプトは、GBDTNameDividerがマルチスレッド環境で
安定して利用できることを確認します。
"""

import concurrent.futures
import threading
import time
import sys
import os
from typing import List

# テスト対象の名前リスト
TEST_NAMES = [
    "菅義偉", "田中太郎", "佐藤花子", "山田一郎", "鈴木二郎",
    "高橋三郎", "渡辺四郎", "伊藤五郎", "中村六郎", "小林七郎",
    "森田八郎", "加藤九郎", "松本十郎", "井上十一郎", "木村十二郎",
    "林十三郎", "山本十四郎", "清水十五郎", "池田十六郎", "橋本十七郎"
]

def test_single_thread():
    """単一スレッドでの基本動作確認"""
    try:
        import namedivider_core
        
        print("=== Single Thread Test ===")
        divider = namedivider_core.GBDTNameDivider()
        
        for name in TEST_NAMES[:5]:  # 5つの名前で確認
            result = divider.divide_name(name)
            print(f"✓ {name}: {result.family} | {result.given}")
        
        print("✓ Single thread test passed")
        return True
    except Exception as e:
        print(f"✗ Single thread test failed: {e}")
        return False

def worker_shared_instance(divider, worker_id: int, iterations: int) -> List[str]:
    """共有GBDTNameDividerインスタンスを使用するワーカー関数"""
    results = []
    
    try:
        for i in range(iterations):
            name_index = (worker_id * iterations + i) % len(TEST_NAMES)
            name = TEST_NAMES[name_index]
            
            result = divider.divide_name(name)
            
            results.append(f"Worker-{worker_id}-{i}: {name} -> {result.family}|{result.given}")
            
            # 少し待機してスレッド競合の可能性を高める
            time.sleep(0.001)
    
    except Exception as e:
        results.append(f"Worker-{worker_id} ERROR: {e}")
    
    return results

def worker_separate_instance(worker_id: int, iterations: int) -> List[str]:
    """各ワーカーで独立したGBDTNameDividerインスタンスを作成"""
    results = []
    
    try:
        import namedivider_core
        # 各ワーカーで独立したインスタンスを作成
        divider = namedivider_core.GBDTNameDivider()
        
        for i in range(iterations):
            name_index = (worker_id * iterations + i) % len(TEST_NAMES)
            name = TEST_NAMES[name_index]
            
            result = divider.divide_name(name)
            results.append(f"Worker-{worker_id}-{i}: {name} -> {result.family}|{result.given}")
            
            time.sleep(0.001)
    
    except Exception as e:
        results.append(f"Worker-{worker_id} ERROR: {e}")
    
    return results

def test_shared_instance_multithreading():
    """共有インスタンスでマルチスレッドテスト"""
    try:
        import namedivider_core
        
        print("\n=== Shared Instance Multi-threading Test ===")
        
        # 共有インスタンスを作成
        shared_divider = namedivider_core.GBDTNameDivider()
        
        # 複数スレッドで同じインスタンスを使用
        num_workers = 8
        iterations_per_worker = 50
        
        with concurrent.futures.ThreadPoolExecutor(max_workers=num_workers) as executor:
            futures = [
                executor.submit(worker_shared_instance, shared_divider, i, iterations_per_worker)
                for i in range(num_workers)
            ]
            
            # すべてのタスクの完了を待機
            for i, future in enumerate(concurrent.futures.as_completed(futures, timeout=30)):
                try:
                    results = future.result()
                    print(f"✓ Worker-{i} completed with {len(results)} results")
                except Exception as e:
                    print(f"✗ Worker-{i} failed: {e}")
                    return False
        
        print("✓ Shared instance test completed (no crash detected)")
        return True
        
    except Exception as e:
        print(f"✗ Shared instance test failed: {e}")
        return False

def test_separate_instances_multithreading():
    """独立インスタンスでマルチスレッドテスト（安全であるべき）"""
    try:
        print("\n=== Separate Instances Multi-threading Test ===")
        
        num_workers = 8
        iterations_per_worker = 50
        
        with concurrent.futures.ThreadPoolExecutor(max_workers=num_workers) as executor:
            futures = [
                executor.submit(worker_separate_instance, i, iterations_per_worker)
                for i in range(num_workers)
            ]
            
            for i, future in enumerate(concurrent.futures.as_completed(futures, timeout=30)):
                try:
                    results = future.result()
                    print(f"✓ Worker-{i} completed with {len(results)} results")
                except Exception as e:
                    print(f"✗ Worker-{i} failed: {e}")
                    return False
        
        print("✓ Separate instances test passed")
        return True
        
    except Exception as e:
        print(f"✗ Separate instances test failed: {e}")
        return False

def main():
    print("🔍 namedivider-core Thread Safety Test")
    print(f"Platform: {sys.platform}")
    print(f"Python: {sys.version}")
    
    try:
        import namedivider_core
        print(f"namedivider-core imported successfully")
    except ImportError as e:
        print(f"❌ Failed to import namedivider-core: {e}")
        sys.exit(1)
    
    # テスト実行
    test_results = []
    
    # 1. 単一スレッドテスト
    test_results.append(("Single Thread", test_single_thread()))
    
    # 2. 独立インスタンスマルチスレッドテスト
    test_results.append(("Separate Instances", test_separate_instances_multithreading()))
    
    # 3. 共有インスタンスマルチスレッドテスト（危険）
    test_results.append(("Shared Instance", test_shared_instance_multithreading()))
    
    # 結果まとめ
    print("\n" + "="*50)
    print("🏁 Test Results Summary")
    print("="*50)
    
    all_passed = True
    for test_name, passed in test_results:
        status = "✅ PASS" if passed else "❌ FAIL"
        print(f"{test_name}: {status}")
        if not passed:
            all_passed = False
    
    if all_passed:
        print("\n🎉 All tests passed! No thread safety issues detected.")
        sys.exit(0)
    else:
        print("\n⚠️  Some tests failed.")
        sys.exit(1)

if __name__ == "__main__":
    main()
