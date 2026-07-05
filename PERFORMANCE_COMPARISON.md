# Performance Comparison: LightGBM binding vs pure Rust evaluator

## Test Environment
- **Old implementation**: `lightgbm-rs` Booster prediction
- **New implementation**: Pure Rust evaluator for the bundled LightGBM text model
- **Test Platform**: Linux x64
- **Rust**: 1.75.0

## Correctness

The pure Rust evaluator was compared against the previous `lightgbm-rs` implementation using the same feature extraction and name-division flow.

| Dataset | Rows | Output differences | Max score difference |
|---------|------|--------------------|----------------------|
| `seimei_test.txt` | 10,000 | 0 | 0.0 |
| `test.txt` | 10,751 | 0 | 0.0 |

## Accuracy

| Dataset | Accuracy |
|---------|----------|
| `seimei_test.txt` | 0.999400000000 |
| `test.txt` | 0.999069853967 |

## Prediction Microbenchmark

20,000 direct model predictions over representative feature vectors:

| Implementation | Time |
|----------------|------|
| Pure Rust evaluator | 157 ms |
| `lightgbm-rs` Booster | 684 ms |

## Summary

The pure Rust evaluator removes the LightGBM native dependency while preserving prediction output exactly for the tested datasets. It also makes `GBDTNameDivider` shareable across threads without relying on thread-local Booster instances.
