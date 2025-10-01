use crate::collector::Collector;
use prometheus::{register_gauge, Gauge};
use std::sync::Mutex;
use sysinfo::System;

pub struct MemoryCollector {
    memory_usage_percent: Gauge,
    memory_total_bytes: Gauge,
    memory_used_bytes: Gauge,
    memory_available_bytes: Gauge,
    system: Mutex<System>,
}

impl MemoryCollector {
    pub fn new() -> Self {
        let memory_usage_percent =
            register_gauge!("memory_usage_percent", "Memory usage in percentage").unwrap();
        let memory_total_bytes =
            register_gauge!("memory_total_bytes", "Total memory in bytes").unwrap();
        let memory_used_bytes =
            register_gauge!("memory_used_bytes", "Used memory in bytes").unwrap();
        let memory_available_bytes =
            register_gauge!("memory_available_bytes", "Available memory in bytes").unwrap();
        let system = Mutex::new(System::new_all());

        MemoryCollector {
            memory_usage_percent,
            memory_total_bytes,
            memory_used_bytes,
            memory_available_bytes,
            system,
        }
    }
}

impl Collector for MemoryCollector {
    fn register_metrics(&self) -> prometheus::Result<()> {
        Ok(())
    }

    fn collect_metrics(&self) {
        let mut system = self.system.lock().unwrap();
        system.refresh_memory();

        let total_memory = system.total_memory();
        let used_memory = system.used_memory();
        let available_memory = system.available_memory();

        // Calculate usage percentage
        let usage_percent = if total_memory > 0 {
            (used_memory as f64 / total_memory as f64) * 100.0
        } else {
            0.0
        };

        self.memory_usage_percent.set(usage_percent);
        self.memory_total_bytes.set(total_memory as f64);
        self.memory_used_bytes.set(used_memory as f64);
        self.memory_available_bytes.set(available_memory as f64);
    }
}
