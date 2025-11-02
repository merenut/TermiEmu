# TermiEmu Benchmarks

This directory contains performance benchmarks for TermiEmu using the Criterion benchmarking framework.

## Running Benchmarks

### Run all benchmarks
```bash
cargo bench
```

### Run a specific benchmark file
```bash
cargo bench --bench basic_benchmark
```

### Run benchmarks with custom filter
```bash
cargo bench -- string_allocation
```

## Benchmark Results

Benchmark results are stored in `target/criterion/` and include:
- HTML reports with graphs and statistics
- Historical data for tracking performance over time
- Detailed timing information and outlier detection

View the results by opening `target/criterion/report/index.html` in a web browser.

## Current Benchmarks

### basic_benchmark.rs
Placeholder benchmarks for infrastructure setup. These will be replaced with actual terminal emulator benchmarks as the project develops:

- **string_operations** - String allocation and concatenation (baseline)
- **vector_operations** - Vector operations simulating grid operations
- **character_operations** - Character processing and width calculation

## Future Benchmarks

As the project develops, benchmarks will be added for:

- **Grid Operations**: Cell updates, line insertion/deletion, scrolling
- **VTE Parser**: ANSI escape sequence parsing throughput
- **Rendering**: Frame rendering time, damage tracking efficiency
- **Font System**: Glyph rasterization and caching
- **PTY I/O**: Read/write throughput and latency
- **Input Handling**: Keyboard and mouse event processing

## Performance Targets

From the PRODUCTION_ROADMAP.md:

- Input latency: < 10ms (target)
- Frame time: < 16ms (60 FPS)
- Startup time: < 200ms
- Memory usage: < 100MB

## CI Integration

Benchmarks can be run in CI for regression detection. Configure GitHub Actions to:
1. Run benchmarks on pull requests
2. Compare against baseline from main branch
3. Alert on performance regressions > 10%

## Tips

- Run benchmarks multiple times to account for variance
- Close other applications to reduce system noise
- Use `cargo bench -- --save-baseline <name>` to save a baseline for comparison
- Compare against baselines with `cargo bench -- --baseline <name>`
