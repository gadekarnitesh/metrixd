use crate::collector::Collector;
use prometheus::{register_counter, register_gauge, register_histogram, Counter, Gauge, Histogram};
use rand::random;
use std::sync::Mutex;
use sysinfo::{Disks, System};

pub struct DiskCollector {
    // Gauge metrics for current disk space
    disk_usage_percent: Gauge,
    disk_total_bytes: Gauge,
    disk_used_bytes: Gauge,
    disk_available_bytes: Gauge,
    disk_inodes_total: Gauge,
    disk_inodes_used: Gauge,

    // Counter metrics for disk I/O operations (cumulative)
    disk_reads_total: Counter,
    disk_writes_total: Counter,
    disk_read_bytes_total: Counter,
    disk_write_bytes_total: Counter,

    // Histogram for disk operation latency simulation
    disk_operation_duration_seconds: Histogram,

    #[allow(dead_code)]
    system: Mutex<System>,
    disks: Mutex<Disks>,
}

impl DiskCollector {
    pub fn new() -> Self {
        // Gauge metrics for current disk space
        let disk_usage_percent = register_gauge!(
            "disk_usage_percent",
            "Disk usage percentage for root filesystem"
        )
        .unwrap();

        let disk_total_bytes = register_gauge!(
            "disk_total_bytes",
            "Total disk space in bytes for root filesystem"
        )
        .unwrap();

        let disk_used_bytes = register_gauge!(
            "disk_used_bytes",
            "Used disk space in bytes for root filesystem"
        )
        .unwrap();

        let disk_available_bytes = register_gauge!(
            "disk_available_bytes",
            "Available disk space in bytes for root filesystem"
        )
        .unwrap();

        let disk_inodes_total = register_gauge!(
            "disk_inodes_total",
            "Total number of inodes on root filesystem"
        )
        .unwrap();

        let disk_inodes_used = register_gauge!(
            "disk_inodes_used",
            "Number of used inodes on root filesystem"
        )
        .unwrap();

        // Counter metrics for disk I/O operations
        let disk_reads_total = register_counter!(
            "disk_reads_total",
            "Total number of disk read operations since start"
        )
        .unwrap();

        let disk_writes_total = register_counter!(
            "disk_writes_total",
            "Total number of disk write operations since start"
        )
        .unwrap();

        let disk_read_bytes_total = register_counter!(
            "disk_read_bytes_total",
            "Total bytes read from disk since start"
        )
        .unwrap();

        let disk_write_bytes_total = register_counter!(
            "disk_write_bytes_total",
            "Total bytes written to disk since start"
        )
        .unwrap();

        // Histogram for disk operation duration
        let disk_operation_duration_seconds = register_histogram!(
            "disk_operation_duration_seconds",
            "Disk operation duration distribution in seconds",
            vec![0.0001, 0.0005, 0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0]
        )
        .unwrap();

        let system = Mutex::new(System::new_all());
        let disks = Mutex::new(Disks::new_with_refreshed_list());

        DiskCollector {
            disk_usage_percent,
            disk_total_bytes,
            disk_used_bytes,
            disk_available_bytes,
            disk_inodes_total,
            disk_inodes_used,
            disk_reads_total,
            disk_writes_total,
            disk_read_bytes_total,
            disk_write_bytes_total,
            disk_operation_duration_seconds,
            system,
            disks,
        }
    }
}

impl Collector for DiskCollector {
    fn register_metrics(&self) -> prometheus::Result<()> {
        Ok(())
    }

    fn collect_metrics(&self) {
        let mut disks = self.disks.lock().unwrap();
        disks.refresh();

        // Get the root filesystem (/) or first available disk
        if let Some(disk) = disks
            .iter()
            .find(|d| d.mount_point().to_str() == Some("/"))
            .or_else(|| disks.iter().next())
        {
            let total_space = disk.total_space();
            let available_space = disk.available_space();
            let used_space = total_space - available_space;
            let usage_percent = if total_space > 0 {
                (used_space as f64 / total_space as f64) * 100.0
            } else {
                0.0
            };

            // Update gauge metrics with real disk data
            self.disk_usage_percent.set(usage_percent);
            self.disk_total_bytes.set(total_space as f64);
            self.disk_used_bytes.set(used_space as f64);
            self.disk_available_bytes.set(available_space as f64);

            // Simulate inode data (real implementation would need filesystem-specific calls)
            let simulated_total_inodes = total_space / 4096; // Rough estimate
            let simulated_used_inodes =
                (simulated_total_inodes as f64 * (usage_percent / 100.0)) as u64;

            self.disk_inodes_total.set(simulated_total_inodes as f64);
            self.disk_inodes_used.set(simulated_used_inodes as f64);
        } else {
            // Fallback values if no disk found
            self.disk_usage_percent.set(45.0);
            self.disk_total_bytes.set(100_000_000_000.0); // 100GB
            self.disk_used_bytes.set(45_000_000_000.0); // 45GB
            self.disk_available_bytes.set(55_000_000_000.0); // 55GB
            self.disk_inodes_total.set(25_600_000.0);
            self.disk_inodes_used.set(11_520_000.0);
        }

        // Simulate disk I/O counters (increment by random amounts for demo)
        // In real implementation, you'd read from /proc/diskstats or similar
        let simulated_reads = (random::<f64>() * 100.0) as f64;
        let simulated_writes = (random::<f64>() * 50.0) as f64;
        let simulated_read_bytes = simulated_reads * 4096.0; // Assume 4KB per read
        let simulated_write_bytes = simulated_writes * 4096.0; // Assume 4KB per write

        self.disk_reads_total.inc_by(simulated_reads);
        self.disk_writes_total.inc_by(simulated_writes);
        self.disk_read_bytes_total.inc_by(simulated_read_bytes);
        self.disk_write_bytes_total.inc_by(simulated_write_bytes);

        // Simulate disk operation latency for histogram
        let simulated_latency = random::<f64>() * 0.1; // 0-100ms
        self.disk_operation_duration_seconds
            .observe(simulated_latency);
    }
}
