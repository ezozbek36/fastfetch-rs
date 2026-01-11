# Benchmarks

This directory contains performance benchmarks for fastfetch-rs using [Criterion](https://github.com/bheisler/criterion.rs).

## Running Benchmarks

Run all benchmarks:
```bash
cargo bench
```

Run specific benchmark:
```bash
cargo bench --bench module_detection
```

Run specific benchmark function:
```bash
cargo bench --bench module_detection -- individual_modules
```

## Viewing Results

After running benchmarks, view the detailed HTML report:
```bash
open target/criterion/report/index.html  # macOS
xdg-open target/criterion/report/index.html  # Linux
```

## Available Benchmarks

### module_detection

- **individual_modules**: Benchmarks each module's detection performance independently
- **parallel_vs_sequential**: Compares parallel vs sequential execution of all modules
- **module_scaling**: Tests performance with 1, 2, 4, and 7 modules
- **full_app**: Benchmarks complete application execution with default config

## Performance Baselines

Establish baselines by running benchmarks on your system. The goal is to:
1. Identify which modules are I/O-bound vs CPU-bound
2. Determine if parallelism provides real benefits
3. Find performance bottlenecks in module detection
4. Track performance regression over time

## Notes

- Benchmarks run on the current system, so results will vary across platforms
- I/O-bound operations may not benefit from parallel execution
- First run will be slower due to system caches
- Results are saved in `target/criterion/` for comparison across runs
