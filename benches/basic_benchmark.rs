//! Basic benchmarks for TermiEmu
//!
//! These benchmarks measure performance of core operations.
//! Run with: `cargo bench`
//!
//! As the project develops, benchmarks should be added for:
//! - Grid operations (cell updates, scrolling)
//! - VTE parser throughput
//! - Rendering performance
//! - Font glyph caching
//! - PTY I/O operations

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

/// Unicode character ranges for width calculation
const EMOJI_START: u32 = 0x1F300;
const CJK_START: u32 = 0x4E00;
const CJK_END: u32 = 0x9FFF;

/// Benchmark string allocation and manipulation
/// This is a placeholder benchmark until we have actual terminal functionality
fn string_operations_benchmark(c: &mut Criterion) {
    c.bench_function("string_allocation", |b| {
        b.iter(|| {
            let s = black_box(String::from("Hello, TermiEmu!"));
            s.len()
        });
    });

    c.bench_function("string_concatenation", |b| {
        b.iter(|| {
            let mut s = String::new();
            for i in 0..black_box(100) {
                s.push_str(&i.to_string());
            }
            s
        });
    });
}

/// Benchmark vector operations (simulating grid operations)
fn vector_operations_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector_operations");

    for size in [80, 160, 320].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut vec = Vec::with_capacity(size);
                for i in 0..black_box(size) {
                    vec.push(i);
                }
                vec
            });
        });
    }

    group.finish();
}

/// Benchmark character operations (simulating text processing)
fn character_operations_benchmark(c: &mut Criterion) {
    c.bench_function("char_is_ascii", |b| {
        let ch = black_box('a');
        b.iter(|| ch.is_ascii());
    });

    c.bench_function("char_to_lowercase", |b| {
        let ch = black_box('A');
        b.iter(|| ch.to_lowercase());
    });

    c.bench_function("unicode_width_calculation", |b| {
        let chars = black_box(vec!['a', 'あ', '🚀', '\u{200D}']);
        b.iter(|| {
            chars
                .iter()
                .map(|c| {
                    // Simplified width calculation
                    if c.is_ascii() {
                        1
                    } else if *c as u32 >= EMOJI_START {
                        2 // Emoji
                    } else if *c as u32 >= CJK_START && *c as u32 <= CJK_END {
                        2 // CJK
                    } else {
                        1
                    }
                })
                .sum::<usize>()
        });
    });
}

// Group all benchmarks
criterion_group!(
    benches,
    string_operations_benchmark,
    vector_operations_benchmark,
    character_operations_benchmark
);

criterion_main!(benches);
