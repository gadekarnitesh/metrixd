
use prometheus::{register_gauge, register_counter, register_histogram, Gauge, Counter, Histogram};
use crate::collector::Collector;
use sysinfo::System;
use std::sync::Mutex;
use rand::random;

pub struct CpuCollector {
    // Gauge for current CPU usage
    cpu_usage: Gauge,
    cpu_cores: Gauge,
    cpu_frequency_mhz: Gauge,

    // Counter for CPU time spent in different modes
    cpu_time_user_seconds_total: Counter,
    cpu_time_system_seconds_total: Counter,
    cpu_time_idle_seconds_total: Counter,

    // Histogram for CPU load distribution
    cpu_load_histogram: Histogram,

    system: Mutex<System>,
}

impl CpuCollector {
    pub fn new() -> Self {
        // Gauge metrics for current CPU state
        let cpu_usage = register_gauge!(
            "cpu_usage_percent",
            "Current CPU usage percentage"
        ).unwrap();

        let cpu_cores = register_gauge!(
            "cpu_cores_total",
            "Total number of CPU cores"
        ).unwrap();

        let cpu_frequency_mhz = register_gauge!(
            "cpu_frequency_mhz",
            "Current CPU frequency in MHz"
        ).unwrap();

        // Counter metrics for CPU time (cumulative)
        let cpu_time_user_seconds_total = register_counter!(
            "cpu_time_user_seconds_total",
            "Total CPU time spent in user mode"
        ).unwrap();

        let cpu_time_system_seconds_total = register_counter!(
            "cpu_time_system_seconds_total",
            "Total CPU time spent in system mode"
        ).unwrap();

        let cpu_time_idle_seconds_total = register_counter!(
            "cpu_time_idle_seconds_total",
            "Total CPU time spent idle"
        ).unwrap();

        // Histogram for CPU load distribution
        let cpu_load_histogram = register_histogram!(
            "cpu_load_distribution",
            "Distribution of CPU load measurements",
            vec![0.0, 10.0, 25.0, 50.0, 75.0, 90.0, 95.0, 99.0, 100.0]
        ).unwrap();

        let system = Mutex::new(System::new_all());

        CpuCollector {
            cpu_usage,
            cpu_cores,
            cpu_frequency_mhz,
            cpu_time_user_seconds_total,
            cpu_time_system_seconds_total,
            cpu_time_idle_seconds_total,
            cpu_load_histogram,
            system,
        }
    }
}

impl Collector for CpuCollector {
    fn register_metrics(&self) -> prometheus::Result<()> {
        Ok(())
    }

    fn collect_metrics(&self) {
        let mut system = self.system.lock().unwrap();
        system.refresh_cpu();

        // Get global CPU usage (average across all cores)
        let cpu_usage = system.global_cpu_info().cpu_usage();

        // Update gauge metrics
        self.cpu_usage.set(cpu_usage as f64);
        self.cpu_cores.set(system.cpus().len() as f64);

        // Get CPU frequency (use first CPU's frequency as representative)
        let cpu_frequency = system.cpus().first()
            .map(|cpu| cpu.frequency())
            .unwrap_or(0) as f64;
        self.cpu_frequency_mhz.set(cpu_frequency);

        // Simulate CPU time counters (in real implementation, read from /proc/stat)
        let simulated_user_time = random::<f64>() * 10.0;
        let simulated_system_time = random::<f64>() * 5.0;
        let simulated_idle_time = random::<f64>() * 100.0;

        self.cpu_time_user_seconds_total.inc_by(simulated_user_time);
        self.cpu_time_system_seconds_total.inc_by(simulated_system_time);
        self.cpu_time_idle_seconds_total.inc_by(simulated_idle_time);

        // Record CPU usage in histogram for distribution analysis
        self.cpu_load_histogram.observe(cpu_usage as f64);
    }
}