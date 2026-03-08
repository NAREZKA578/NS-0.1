//! Profiler module
//! 
//! Contains performance profiling utilities

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Profile entry for a single measurement
#[derive(Debug, Clone)]
pub struct ProfileEntry {
    /// Name of the profiled section
    pub name: String,
    /// Total time spent
    pub total_time: Duration,
    /// Number of calls
    pub call_count: u64,
    /// Minimum time
    pub min_time: Duration,
    /// Maximum time
    pub max_time: Duration,
}

impl ProfileEntry {
    pub fn new(name: String) -> Self {
        Self {
            name,
            total_time: Duration::ZERO,
            call_count: 0,
            min_time: Duration::MAX,
            max_time: Duration::ZERO,
        }
    }
    
    /// Record a new measurement
    pub fn record(&mut self, duration: Duration) {
        self.total_time += duration;
        self.call_count += 1;
        
        if duration < self.min_time {
            self.min_time = duration;
        }
        if duration > self.max_time {
            self.max_time = duration;
        }
    }
    
    /// Get average time
    pub fn average(&self) -> Duration {
        if self.call_count == 0 {
            return Duration::ZERO;
        }
        self.total_time / self.call_count as u32
    }
    
    /// Get time in milliseconds
    pub fn total_ms(&self) -> f64 {
        self.total_time.as_secs_f64() * 1000.0
    }
    
    /// Get average in milliseconds
    pub fn average_ms(&self) -> f64 {
        self.average().as_secs_f64() * 1000.0
    }
    
    /// Get min in milliseconds
    pub fn min_ms(&self) -> f64 {
        self.min_time.as_secs_f64() * 1000.0
    }
    
    /// Get max in milliseconds
    pub fn max_ms(&self) -> f64 {
        self.max_time.as_secs_f64() * 1000.0
    }
}

/// Profiler manager
pub struct Profiler {
    /// All profile entries
    entries: HashMap<String, ProfileEntry>,
    /// Is profiling enabled
    enabled: bool,
    /// Frame start time
    frame_start: Option<Instant>,
    /// Frame times for FPS calculation
    frame_times: Vec<f32>,
    /// Maximum frame times to track
    max_frame_times: usize,
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Profiler {
    /// Create new profiler
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            enabled: true,
            frame_start: None,
            frame_times: Vec::new(),
            max_frame_times: 100,
        }
    }
    
    /// Enable/disable profiling
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Check if profiling is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Start profiling a section
    pub fn begin(&mut self, name: &str) -> ProfileScope {
        if !self.enabled {
            return ProfileScope::disabled();
        }
        
        ProfileScope::new(self, name.to_string())
    }
    
    /// Internal method to record a measurement
    pub fn record(&mut self, name: String, duration: Duration) {
        let entry = self.entries
            .entry(name.clone())
            .or_insert_with(|| ProfileEntry::new(name));
        entry.record(duration);
    }
    
    /// Start a new frame
    pub fn begin_frame(&mut self) {
        if !self.enabled {
            return;
        }
        self.frame_start = Some(Instant::now());
    }
    
    /// End current frame and record frame time
    pub fn end_frame(&mut self) -> f32 {
        if !self.enabled {
            return 0.0;
        }
        
        if let Some(start) = self.frame_start.take() {
            let frame_time = start.elapsed().as_secs_f32();
            
            self.frame_times.push(frame_time);
            if self.frame_times.len() > self.max_frame_times {
                self.frame_times.remove(0);
            }
            
            frame_time
        } else {
            0.0
        }
    }
    
    /// Get current FPS
    pub fn get_fps(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        
        let avg_frame_time: f32 = self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32;
        if avg_frame_time < f32::EPSILON {
            return 0.0;
        }
        
        1.0 / avg_frame_time
    }
    
    /// Get average frame time in ms
    pub fn get_avg_frame_time_ms(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        
        (self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32) * 1000.0
    }
    
    /// Get entry by name
    pub fn get_entry(&self, name: &str) -> Option<&ProfileEntry> {
        self.entries.get(name)
    }
    
    /// Print all profile results
    pub fn print_results(&self) {
        if !self.enabled {
            println!("Profiler is disabled");
            return;
        }
        
        println!("\n=== PROFILER RESULTS ===");
        println!("FPS: {:.1}", self.get_fps());
        println!("Frame time: {:.2} ms", self.get_avg_frame_time_ms());
        println!();
        
        let mut entries: Vec<&ProfileEntry> = self.entries.values().collect();
        entries.sort_by(|a, b| b.total_time.cmp(&a.total_time));
        
        for entry in entries {
            println!(
                "{:<30} | Calls: {:>6} | Avg: {:>8.3} ms | Min: {:>8.3} ms | Max: {:>8.3} ms | Total: {:>8.3} ms",
                entry.name,
                entry.call_count,
                entry.average_ms(),
                if entry.min_time == Duration::MAX { 0.0 } else { entry.min_ms() },
                entry.max_ms(),
                entry.total_ms()
            );
        }
        println!("========================\n");
    }
    
    /// Reset all statistics
    pub fn reset(&mut self) {
        self.entries.clear();
        self.frame_times.clear();
        self.frame_start = None;
    }
    
    /// Reset specific entry
    pub fn reset_entry(&mut self, name: &str) {
        if let Some(entry) = self.entries.get_mut(name) {
            entry.total_time = Duration::ZERO;
            entry.call_count = 0;
            entry.min_time = Duration::MAX;
            entry.max_time = Duration::ZERO;
        }
    }
}

/// RAII-style profile scope guard
pub struct ProfileScope {
    profiler: Option<*mut Profiler>,
    name: String,
    start: Option<Instant>,
}

impl ProfileScope {
    /// Create a disabled scope
    pub fn disabled() -> Self {
        Self {
            profiler: None,
            name: String::new(),
            start: None,
        }
    }
    
    /// Create new profile scope
    pub fn new(profiler: &mut Profiler, name: String) -> Self {
        Self {
            profiler: Some(profiler as *mut Profiler),
            name,
            start: Some(Instant::now()),
        }
    }
}

impl Drop for ProfileScope {
    fn drop(&mut self) {
        if let (Some(profiler), Some(start)) = (self.profiler, self.start) {
            unsafe {
                (*profiler).record(self.name.clone(), start.elapsed());
            }
        }
    }
}

/// Macro for easy profiling
#[macro_export]
macro_rules! profile_scope {
    ($profiler:expr, $name:expr) => {
        let _profile_guard = $profiler.begin($name);
    };
}

#[macro_export]
macro_rules! profile_function {
    ($profiler:expr) => {
        profile_scope!($profiler, stringify!($profiler));
    };
}
