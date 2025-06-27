#!/usr/bin/env python3
"""
Test script for namedivider_core package
Used by CI/CD to validate wheel functionality
"""

import namedivider_core

def main():
    print('Module imported successfully')
    
    # Test Basic divider
    basic = namedivider_core.BasicNameDivider()
    result1 = basic.divide_name('田中太郎')
    assert str(result1) == '田中 太郎'
    print('BasicNameDivider test passed')
    
    # Test GBDT divider
    gbdt = namedivider_core.GBDTNameDivider()
    result2 = gbdt.divide_name('佐藤花子')
    assert str(result2) == '佐藤 花子'
    print('GBDTNameDivider test passed')
    
    print('All tests passed!')

if __name__ == '__main__':
    main()