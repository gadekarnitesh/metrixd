use crate::collector::Collector;
use prometheus::{register_gauge, Gauge};
use std::sync::Mutex;
use sysinfo::System;

pub struct SystemCollector {
    load_average_1min: Gauge,
    load_average_5min: Gauge,
    load_average_15min: Gauge,
    uptime_seconds: Gauge,
    process_count: Gauge,
    system: Mutex<System>,
}

impl SystemCollector {
    pub fn new() -> Self {
        let load_average_1min =
            register_gauge!("load_average_1min", "System load average over 1 minute").unwrap();
        let load_average_5min =
            register_gauge!("load_average_5min", "System load average over 5 minutes").unwrap();
        let load_average_15min =
            register_gauge!("load_average_15min", "System load average over 15 minutes").unwrap();
        let uptime_seconds = register_gauge!("uptime_seconds", "System uptime in seconds").unwrap();
        let process_count =
            register_gauge!("process_count", "Number of running processes").unwrap();
        let system = Mutex::new(System::new_all());

        SystemCollector {
            load_average_1min,
            load_average_5min,
            load_average_15min,
            uptime_seconds,
            process_count,
            system,
        }
    }
}

impl Collector for SystemCollector {
    fn register_metrics(&self) -> prometheus::Result<()> {
        Ok(())
    }

    fn collect_metrics(&self) {
        let mut system = self.system.lock().unwrap();
        system.refresh_all();

        // Get load averages (static method)
        let load_avg = System::load_average();
        self.load_average_1min.set(load_avg.one);
        self.load_average_5min.set(load_avg.five);
        self.load_average_15min.set(load_avg.fifteen);

        // Get uptime (static method)
        self.uptime_seconds.set(System::uptime() as f64);

        // Get process count
        self.process_count.set(system.processes().len() as f64);
    }
}
