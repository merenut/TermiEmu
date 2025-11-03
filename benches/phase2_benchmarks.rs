//! Phase 2 Performance Benchmarks (US-045)
//!
//! Comprehensive performance benchmarking suite for TermiEmu.
//! Measures:
//! - Grid operations (scrolling, updates, rendering)
//! - VTE parser throughput
//! - Search performance
//! - Memory usage characteristics
//!
//! Run with: `cargo bench --bench phase2_benchmarks`

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use termiemu::terminal::{
    cell::Cell,
    grid::Grid,
    parser::Parser,
    search::{SearchOptions, Searcher},
};

// ============================================================================
// Grid Operations Benchmarks
// ============================================================================

fn bench_grid_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_creation");
    
    for (cols, rows) in [(80, 24), (160, 48), (320, 96)].iter() {
        let id = format!("{}x{}", cols, rows);
        group.bench_with_input(BenchmarkId::from_parameter(&id), &(cols, rows), |b, (cols, rows)| {
            b.iter(|| Grid::new(black_box(**cols), black_box(**rows), 10_000));
        });
    }
    
    group.finish();
}

fn bench_grid_cell_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_cell_update");
    
    let mut grid = Grid::new(80, 24, 10_000);
    let cell = Cell::new('A');
    
    group.bench_function("single_cell", |b| {
        b.iter(|| {
            grid.set(black_box(40), black_box(12), black_box(cell.clone()));
        });
    });
    
    group.bench_function("full_row", |b| {
        b.iter(|| {
            for col in 0..80 {
                grid.set(black_box(col), black_box(12), black_box(cell.clone()));
            }
        });
    });
    
    group.bench_function("full_screen", |b| {
        b.iter(|| {
            for row in 0..24 {
                for col in 0..80 {
                    grid.set(black_box(col), black_box(row), black_box(cell.clone()));
                }
            }
        });
    });
    
    group.finish();
}

fn bench_grid_scrolling(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_scrolling");
    
    let mut grid = Grid::new(80, 24, 10_000);
    
    // Fill grid with some content
    for row in 0..24 {
        for col in 0..80 {
            grid.set(col, row, Cell::new('X'));
        }
    }
    
    group.bench_function("scroll_up_1_line", |b| {
        b.iter(|| {
            grid.scroll_up(black_box(1));
        });
    });
    
    group.bench_function("scroll_up_10_lines", |b| {
        b.iter(|| {
            grid.scroll_up(black_box(10));
        });
    });
    
    group.bench_function("scroll_down_1_line", |b| {
        b.iter(|| {
            grid.scroll_down(black_box(1));
        });
    });
    
    group.finish();
}

fn bench_grid_clear_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_clear");
    
    let mut grid = Grid::new(80, 24, 10_000);
    
    // Fill grid
    for row in 0..24 {
        for col in 0..80 {
            grid.set(col, row, Cell::new('X'));
        }
    }
    
    group.bench_function("clear_row", |b| {
        b.iter(|| {
            grid.clear_row(black_box(12));
        });
    });
    
    group.bench_function("clear_full_grid", |b| {
        b.iter(|| {
            grid.clear();
        });
    });
    
    group.finish();
}

// ============================================================================
// VTE Parser Benchmarks
// ============================================================================

fn bench_parser_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_throughput");
    
    // Simple text
    let simple_text = b"Hello, World! This is a test.\n".repeat(100);
    group.throughput(Throughput::Bytes(simple_text.len() as u64));
    group.bench_function("simple_text", |b| {
        let mut parser = Parser::new(80, 24);
        b.iter(|| {
            parser.advance_bytes(black_box(&simple_text));
        });
    });
    
    // Text with colors
    let colored_text = b"\x1b[31mRed\x1b[32mGreen\x1b[34mBlue\x1b[0m".repeat(100);
    group.throughput(Throughput::Bytes(colored_text.len() as u64));
    group.bench_function("colored_text", |b| {
        let mut parser = Parser::new(80, 24);
        b.iter(|| {
            parser.advance_bytes(black_box(&colored_text));
        });
    });
    
    // Text with cursor movement
    let cursor_text = b"\x1b[H\x1b[2JHello\x1b[10;20HWorld".repeat(50);
    group.throughput(Throughput::Bytes(cursor_text.len() as u64));
    group.bench_function("cursor_movement", |b| {
        let mut parser = Parser::new(80, 24);
        b.iter(|| {
            parser.advance_bytes(black_box(&cursor_text));
        });
    });
    
    // Text with hyperlinks
    let hyperlink_text = b"\x1b]8;;http://example.com\x1b\\Link Text\x1b]8;;\x1b\\".repeat(50);
    group.throughput(Throughput::Bytes(hyperlink_text.len() as u64));
    group.bench_function("hyperlinks", |b| {
        let mut parser = Parser::new(80, 24);
        b.iter(|| {
            parser.advance_bytes(black_box(&hyperlink_text));
        });
    });
    
    group.finish();
}

fn bench_parser_single_bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_single_bytes");
    
    let mut parser = Parser::new(80, 24);
    
    group.bench_function("printable_char", |b| {
        b.iter(|| {
            parser.advance(black_box(b'A'));
        });
    });
    
    group.bench_function("control_char", |b| {
        b.iter(|| {
            parser.advance(black_box(0x0A)); // Line feed
        });
    });
    
    group.finish();
}

// ============================================================================
// Search Benchmarks
// ============================================================================

fn bench_search_literal(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_literal");
    
    // Create a grid with repeated content
    let mut grid = Grid::new(80, 100, 1000);
    for row in 0..100 {
        let text = format!("This is line {} with some searchable content", row);
        for (col, ch) in text.chars().enumerate() {
            if col < 80 {
                grid.set(col, row, Cell::new(ch));
            }
        }
    }
    
    group.bench_function("single_match", |b| {
        let mut searcher = Searcher::new();
        b.iter(|| {
            searcher.search(black_box(&grid), black_box("searchable"), SearchOptions::new()).unwrap();
        });
    });
    
    group.bench_function("many_matches", |b| {
        let mut searcher = Searcher::new();
        b.iter(|| {
            searcher.search(black_box(&grid), black_box("line"), SearchOptions::new()).unwrap();
        });
    });
    
    group.bench_function("no_matches", |b| {
        let mut searcher = Searcher::new();
        b.iter(|| {
            searcher.search(black_box(&grid), black_box("xyznotfound"), SearchOptions::new()).unwrap();
        });
    });
    
    group.finish();
}

fn bench_search_regex(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_regex");
    
    let mut grid = Grid::new(80, 100, 1000);
    for row in 0..100 {
        let text = format!("Line {} with numbers: 123, 456, 789", row);
        for (col, ch) in text.chars().enumerate() {
            if col < 80 {
                grid.set(col, row, Cell::new(ch));
            }
        }
    }
    
    let options = SearchOptions::new().use_regex(true);
    
    group.bench_function("simple_pattern", |b| {
        let mut searcher = Searcher::new();
        b.iter(|| {
            searcher.search(black_box(&grid), black_box(r"\d+"), black_box(options.clone())).unwrap();
        });
    });
    
    group.bench_function("word_boundary", |b| {
        let mut searcher = Searcher::new();
        b.iter(|| {
            searcher.search(black_box(&grid), black_box(r"\bLine\b"), black_box(options.clone())).unwrap();
        });
    });
    
    group.finish();
}

fn bench_search_navigation(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_navigation");
    
    let mut grid = Grid::new(80, 50, 1000);
    for row in 0..50 {
        let text = "test ".repeat(16); // Multiple matches per line
        for (col, ch) in text.chars().enumerate() {
            if col < 80 {
                grid.set(col, row, Cell::new(ch));
            }
        }
    }
    
    let mut searcher = Searcher::new();
    searcher.search(&grid, "test", SearchOptions::new()).unwrap();
    
    group.bench_function("next_match", |b| {
        b.iter(|| {
            black_box(searcher.next_match());
        });
    });
    
    group.bench_function("previous_match", |b| {
        b.iter(|| {
            black_box(searcher.previous_match());
        });
    });
    
    group.finish();
}

// ============================================================================
// Cell Operations Benchmarks
// ============================================================================

fn bench_cell_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("cell_operations");
    
    group.bench_function("cell_creation", |b| {
        b.iter(|| {
            Cell::new(black_box('A'))
        });
    });
    
    group.bench_function("cell_clone", |b| {
        let cell = Cell::new('A');
        b.iter(|| {
            black_box(cell.clone())
        });
    });
    
    let cell = Cell::new('A');
    group.bench_function("cell_has_hyperlink", |b| {
        b.iter(|| {
            black_box(cell.has_hyperlink())
        });
    });
    
    group.finish();
}

// ============================================================================
// Memory and Scalability Benchmarks
// ============================================================================

fn bench_large_grids(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_grids");
    
    // Simulate large terminal sizes
    for (cols, rows) in [(160, 48), (320, 96), (400, 120)].iter() {
        let id = format!("grid_{}x{}", cols, rows);
        group.bench_with_input(BenchmarkId::from_parameter(&id), &(cols, rows), |b, (cols, rows)| {
            b.iter(|| {
                let mut grid = Grid::new(black_box(**cols), black_box(**rows), 10_000);
                // Fill the grid
                for row in 0..(**rows).min(24) {
                    for col in 0..(**cols).min(80) {
                        grid.set(col, row, Cell::new('X'));
                    }
                }
                grid
            });
        });
    }
    
    group.finish();
}

fn bench_scrollback_buffer(c: &mut Criterion) {
    let mut group = c.benchmark_group("scrollback_buffer");
    
    // Measure scrollback performance with different sizes
    for scrollback_size in [1_000, 10_000, 50_000].iter() {
        let id = format!("scrollback_{}", scrollback_size);
        group.bench_with_input(BenchmarkId::from_parameter(&id), scrollback_size, |b, &size| {
            b.iter(|| {
                let mut grid = Grid::new(80, 24, black_box(size));
                // Generate scrollback
                for _ in 0..size {
                    grid.scroll_up(1);
                }
                grid
            });
        });
    }
    
    group.finish();
}

// Group all benchmarks
criterion_group!(
    benches,
    bench_grid_creation,
    bench_grid_cell_update,
    bench_grid_scrolling,
    bench_grid_clear_operations,
    bench_parser_throughput,
    bench_parser_single_bytes,
    bench_search_literal,
    bench_search_regex,
    bench_search_navigation,
    bench_cell_operations,
    bench_large_grids,
    bench_scrollback_buffer,
);

criterion_main!(benches);
