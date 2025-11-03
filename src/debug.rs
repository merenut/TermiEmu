//! Debug mode and diagnostics
//!
//! Provides debug overlays, performance metrics, and diagnostic information
//! for debugging and profiling TermiEmu.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Performance metrics collector
#[derive(Clone, Debug)]
pub struct PerformanceMetrics {
    /// Frame times (milliseconds)
    frame_times: VecDeque<f64>,
    /// Maximum number of frame times to keep
    max_samples: usize,
    /// Last frame start time
    last_frame_start: Option<Instant>,
    /// Total frames rendered
    total_frames: u64,
    /// Total bytes processed by PTY
    total_pty_bytes: u64,
    /// Parser sequence counts
    parser_sequences: ParserStats,
}

/// Parser statistics
#[derive(Clone, Debug, Default)]
pub struct ParserStats {
    /// Number of printable characters processed
    pub print_chars: u64,
    /// Number of control sequences (ESC, CSI, OSC, etc.)
    pub control_sequences: u64,
    /// Number of CSI sequences
    pub csi_sequences: u64,
    /// Number of OSC sequences
    pub osc_sequences: u64,
    /// Number of hyperlinks processed
    pub hyperlinks: u64,
}

impl PerformanceMetrics {
    /// Create a new performance metrics collector
    pub fn new(max_samples: usize) -> Self {
        Self {
            frame_times: VecDeque::with_capacity(max_samples),
            max_samples,
            last_frame_start: None,
            total_frames: 0,
            total_pty_bytes: 0,
            parser_sequences: ParserStats::default(),
        }
    }

    /// Start a new frame
    pub fn start_frame(&mut self) {
        self.last_frame_start = Some(Instant::now());
    }

    /// End the current frame and record its duration
    pub fn end_frame(&mut self) {
        if let Some(start) = self.last_frame_start.take() {
            let duration = start.elapsed();
            let ms = duration.as_secs_f64() * 1000.0;
            
            self.frame_times.push_back(ms);
            if self.frame_times.len() > self.max_samples {
                self.frame_times.pop_front();
            }
            
            self.total_frames += 1;
        }
    }

    /// Get the current FPS (frames per second)
    pub fn fps(&self) -> f64 {
        let avg_frame_time = self.average_frame_time();
        if avg_frame_time > 0.0 {
            1000.0 / avg_frame_time
        } else {
            0.0
        }
    }

    /// Get the average frame time in milliseconds
    pub fn average_frame_time(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.frame_times.iter().sum();
        sum / self.frame_times.len() as f64
    }

    /// Get the minimum frame time in milliseconds
    pub fn min_frame_time(&self) -> f64 {
        self.frame_times.iter().copied().fold(f64::INFINITY, f64::min)
    }

    /// Get the maximum frame time in milliseconds
    pub fn max_frame_time(&self) -> f64 {
        self.frame_times.iter().copied().fold(f64::NEG_INFINITY, f64::max)
    }

    /// Get the 99th percentile frame time
    pub fn p99_frame_time(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        
        let mut sorted: Vec<f64> = self.frame_times.iter().copied().collect();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        let index = (sorted.len() as f64 * 0.99) as usize;
        sorted.get(index.min(sorted.len() - 1)).copied().unwrap_or(0.0)
    }

    /// Record PTY bytes processed
    pub fn record_pty_bytes(&mut self, bytes: u64) {
        self.total_pty_bytes += bytes;
    }

    /// Get total PTY bytes processed
    pub fn total_pty_bytes(&self) -> u64 {
        self.total_pty_bytes
    }

    /// Get total frames rendered
    pub fn total_frames(&self) -> u64 {
        self.total_frames
    }

    /// Get parser statistics
    pub fn parser_stats(&self) -> &ParserStats {
        &self.parser_sequences
    }

    /// Get mutable parser statistics
    pub fn parser_stats_mut(&mut self) -> &mut ParserStats {
        &mut self.parser_sequences
    }

    /// Reset all metrics
    pub fn reset(&mut self) {
        self.frame_times.clear();
        self.last_frame_start = None;
        self.total_frames = 0;
        self.total_pty_bytes = 0;
        self.parser_sequences = ParserStats::default();
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new(120) // Keep last 120 frames (2 seconds at 60fps)
    }
}

/// Debug overlay configuration
#[derive(Clone, Debug)]
pub struct DebugOverlay {
    /// Whether the overlay is visible
    pub visible: bool,
    /// Show FPS counter
    pub show_fps: bool,
    /// Show frame time statistics
    pub show_frame_times: bool,
    /// Show memory usage
    pub show_memory: bool,
    /// Show PTY throughput
    pub show_pty_throughput: bool,
    /// Show parser statistics
    pub show_parser_stats: bool,
    /// Show grid information
    pub show_grid_info: bool,
}

impl Default for DebugOverlay {
    fn default() -> Self {
        Self {
            visible: false,
            show_fps: true,
            show_frame_times: true,
            show_memory: true,
            show_pty_throughput: true,
            show_parser_stats: false,
            show_grid_info: false,
        }
    }
}

impl DebugOverlay {
    /// Create a new debug overlay with all features enabled
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable all debug features
    pub fn enable_all(&mut self) {
        self.visible = true;
        self.show_fps = true;
        self.show_frame_times = true;
        self.show_memory = true;
        self.show_pty_throughput = true;
        self.show_parser_stats = true;
        self.show_grid_info = true;
    }

    /// Disable all debug features
    pub fn disable_all(&mut self) {
        self.visible = false;
    }

    /// Toggle overlay visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Format metrics as text for display
    pub fn format_metrics(&self, metrics: &PerformanceMetrics) -> Vec<String> {
        let mut lines = Vec::new();

        if !self.visible {
            return lines;
        }

        lines.push("=== TermiEmu Debug Overlay ===".to_string());

        if self.show_fps {
            lines.push(format!("FPS: {:.1}", metrics.fps()));
        }

        if self.show_frame_times {
            lines.push(format!("Frame Time (ms):"));
            lines.push(format!("  Avg: {:.2}", metrics.average_frame_time()));
            lines.push(format!("  Min: {:.2}", metrics.min_frame_time()));
            lines.push(format!("  Max: {:.2}", metrics.max_frame_time()));
            lines.push(format!("  P99: {:.2}", metrics.p99_frame_time()));
        }

        if self.show_memory {
            // Get current memory usage (simplified)
            lines.push(format!("Memory:"));
            lines.push(format!("  (detailed stats require system APIs)"));
        }

        if self.show_pty_throughput {
            lines.push(format!("PTY:"));
            lines.push(format!("  Total bytes: {}", metrics.total_pty_bytes()));
            if metrics.total_frames() > 0 {
                let bytes_per_frame = metrics.total_pty_bytes() as f64 / metrics.total_frames() as f64;
                lines.push(format!("  Avg bytes/frame: {:.1}", bytes_per_frame));
            }
        }

        if self.show_parser_stats {
            let stats = metrics.parser_stats();
            lines.push(format!("Parser:"));
            lines.push(format!("  Print chars: {}", stats.print_chars));
            lines.push(format!("  Control seqs: {}", stats.control_sequences));
            lines.push(format!("  CSI seqs: {}", stats.csi_sequences));
            lines.push(format!("  OSC seqs: {}", stats.osc_sequences));
            lines.push(format!("  Hyperlinks: {}", stats.hyperlinks));
        }

        lines.push("=============================".to_string());
        lines
    }
}

/// Latency measurement tool
#[derive(Clone, Debug)]
pub struct LatencyMeasurement {
    /// Input timestamps for latency tracking
    input_timestamps: VecDeque<Instant>,
    /// Maximum number of timestamps to keep
    max_samples: usize,
    /// Total latency measurements
    latencies: VecDeque<Duration>,
}

impl LatencyMeasurement {
    /// Create a new latency measurement tool
    pub fn new(max_samples: usize) -> Self {
        Self {
            input_timestamps: VecDeque::with_capacity(max_samples),
            latencies: VecDeque::with_capacity(max_samples),
            max_samples,
        }
    }

    /// Record an input event
    pub fn record_input(&mut self) {
        self.input_timestamps.push_back(Instant::now());
        if self.input_timestamps.len() > self.max_samples {
            self.input_timestamps.pop_front();
        }
    }

    /// Record when the input was processed and displayed
    pub fn record_display(&mut self) {
        if let Some(input_time) = self.input_timestamps.pop_front() {
            let latency = input_time.elapsed();
            self.latencies.push_back(latency);
            if self.latencies.len() > self.max_samples {
                self.latencies.pop_front();
            }
        }
    }

    /// Get average latency in milliseconds
    pub fn average_latency_ms(&self) -> f64 {
        if self.latencies.is_empty() {
            return 0.0;
        }
        let sum: Duration = self.latencies.iter().sum();
        sum.as_secs_f64() * 1000.0 / self.latencies.len() as f64
    }

    /// Get 99th percentile latency in milliseconds
    pub fn p99_latency_ms(&self) -> f64 {
        if self.latencies.is_empty() {
            return 0.0;
        }
        
        let mut sorted: Vec<f64> = self.latencies
            .iter()
            .map(|d| d.as_secs_f64() * 1000.0)
            .collect();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        let index = (sorted.len() as f64 * 0.99) as usize;
        sorted.get(index.min(sorted.len() - 1)).copied().unwrap_or(0.0)
    }

    /// Check if latency target is met
    pub fn meets_target(&self, target_ms: f64) -> bool {
        self.average_latency_ms() <= target_ms && self.p99_latency_ms() <= target_ms * 1.5
    }
}

impl Default for LatencyMeasurement {
    fn default() -> Self {
        Self::new(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_performance_metrics_frame_timing() {
        let mut metrics = PerformanceMetrics::new(10);
        
        // Simulate some frames
        for _ in 0..5 {
            metrics.start_frame();
            thread::sleep(Duration::from_millis(10));
            metrics.end_frame();
        }
        
        assert_eq!(metrics.total_frames(), 5);
        assert!(metrics.average_frame_time() >= 10.0);
        assert!(metrics.fps() > 0.0);
    }

    #[test]
    fn test_performance_metrics_pty_bytes() {
        let mut metrics = PerformanceMetrics::new(10);
        
        metrics.record_pty_bytes(1024);
        metrics.record_pty_bytes(2048);
        
        assert_eq!(metrics.total_pty_bytes(), 3072);
    }

    #[test]
    fn test_performance_metrics_parser_stats() {
        let mut metrics = PerformanceMetrics::new(10);
        
        let stats = metrics.parser_stats_mut();
        stats.print_chars = 100;
        stats.csi_sequences = 10;
        stats.hyperlinks = 2;
        
        assert_eq!(metrics.parser_stats().print_chars, 100);
        assert_eq!(metrics.parser_stats().csi_sequences, 10);
        assert_eq!(metrics.parser_stats().hyperlinks, 2);
    }

    #[test]
    fn test_performance_metrics_percentiles() {
        let mut metrics = PerformanceMetrics::new(100);
        
        // Add known frame times
        for _ in 0..90 {
            metrics.frame_times.push_back(10.0);
        }
        for _ in 0..10 {
            metrics.frame_times.push_back(50.0);
        }
        
        let p99 = metrics.p99_frame_time();
        assert!(p99 >= 45.0 && p99 <= 50.0);
    }

    #[test]
    fn test_debug_overlay_toggle() {
        let mut overlay = DebugOverlay::new();
        
        assert!(!overlay.visible);
        
        overlay.toggle();
        assert!(overlay.visible);
        
        overlay.toggle();
        assert!(!overlay.visible);
    }

    #[test]
    fn test_debug_overlay_format_metrics() {
        let overlay = DebugOverlay {
            visible: true,
            show_fps: true,
            show_frame_times: false,
            show_memory: false,
            show_pty_throughput: false,
            show_parser_stats: false,
            show_grid_info: false,
        };
        
        let mut metrics = PerformanceMetrics::new(10);
        metrics.start_frame();
        thread::sleep(Duration::from_millis(10));
        metrics.end_frame();
        
        let lines = overlay.format_metrics(&metrics);
        assert!(!lines.is_empty());
        assert!(lines.iter().any(|l| l.contains("FPS")));
    }

    #[test]
    fn test_latency_measurement() {
        let mut latency = LatencyMeasurement::new(10);
        
        latency.record_input();
        thread::sleep(Duration::from_millis(5));
        latency.record_display();
        
        let avg = latency.average_latency_ms();
        assert!(avg >= 5.0);
    }

    #[test]
    fn test_latency_target() {
        let mut latency = LatencyMeasurement::new(10);
        
        // Simulate low latency inputs
        for _ in 0..5 {
            latency.record_input();
            thread::sleep(Duration::from_millis(5));
            latency.record_display();
        }
        
        // Should meet 10ms target
        assert!(latency.meets_target(10.0));
        
        // Should not meet 2ms target
        assert!(!latency.meets_target(2.0));
    }

    #[test]
    fn test_metrics_reset() {
        let mut metrics = PerformanceMetrics::new(10);
        
        metrics.start_frame();
        metrics.end_frame();
        metrics.record_pty_bytes(1000);
        metrics.parser_stats_mut().print_chars = 50;
        
        assert!(metrics.total_frames() > 0);
        assert!(metrics.total_pty_bytes() > 0);
        
        metrics.reset();
        
        assert_eq!(metrics.total_frames(), 0);
        assert_eq!(metrics.total_pty_bytes(), 0);
        assert_eq!(metrics.parser_stats().print_chars, 0);
    }
}
