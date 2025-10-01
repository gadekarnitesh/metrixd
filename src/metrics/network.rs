use crate::collector::Collector;
use prometheus::{register_counter, register_gauge, register_histogram, Counter, Gauge, Histogram};
use std::sync::Mutex;
use sysinfo::{Networks, System};

pub struct NetworkCollector {
    // Gauges for current values
    network_bytes_received: Gauge,
    network_bytes_transmitted: Gauge,
    network_packets_received: Gauge,
    network_packets_transmitted: Gauge,
    network_errors_received: Gauge,
    network_errors_transmitted: Gauge,

    // Counters for cumulative values (always increasing)
    network_bytes_received_total: Counter,
    network_bytes_transmitted_total: Counter,
    network_packets_received_total: Counter,
    network_packets_transmitted_total: Counter,

    // Histogram for network latency simulation (for learning)
    network_latency_histogram: Histogram,

    #[allow(dead_code)]
    system: Mutex<System>,
    networks: Mutex<Networks>,
}

impl NetworkCollector {
    pub fn new() -> Self {
        // Gauge metrics (current snapshot values)
        let network_bytes_received = register_gauge!(
            "network_bytes_received",
            "Current network bytes received per second"
        )
        .unwrap();

        let network_bytes_transmitted = register_gauge!(
            "network_bytes_transmitted",
            "Current network bytes transmitted per second"
        )
        .unwrap();

        let network_packets_received = register_gauge!(
            "network_packets_received",
            "Current network packets received per second"
        )
        .unwrap();

        let network_packets_transmitted = register_gauge!(
            "network_packets_transmitted",
            "Current network packets transmitted per second"
        )
        .unwrap();

        let network_errors_received =
            register_gauge!("network_errors_received", "Current network errors received").unwrap();

        let network_errors_transmitted = register_gauge!(
            "network_errors_transmitted",
            "Current network errors transmitted"
        )
        .unwrap();

        // Counter metrics (cumulative, always increasing)
        let network_bytes_received_total = register_counter!(
            "network_bytes_received_total",
            "Total network bytes received since start"
        )
        .unwrap();

        let network_bytes_transmitted_total = register_counter!(
            "network_bytes_transmitted_total",
            "Total network bytes transmitted since start"
        )
        .unwrap();

        let network_packets_received_total = register_counter!(
            "network_packets_received_total",
            "Total network packets received since start"
        )
        .unwrap();

        let network_packets_transmitted_total = register_counter!(
            "network_packets_transmitted_total",
            "Total network packets transmitted since start"
        )
        .unwrap();

        // Histogram metric (for distribution of values)
        let network_latency_histogram = register_histogram!(
            "network_latency_seconds",
            "Network latency distribution in seconds",
            vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
        )
        .unwrap();

        let system = Mutex::new(System::new_all());
        let networks = Mutex::new(Networks::new_with_refreshed_list());

        NetworkCollector {
            network_bytes_received,
            network_bytes_transmitted,
            network_packets_received,
            network_packets_transmitted,
            network_errors_received,
            network_errors_transmitted,
            network_bytes_received_total,
            network_bytes_transmitted_total,
            network_packets_received_total,
            network_packets_transmitted_total,
            network_latency_histogram,
            system,
            networks,
        }
    }
}

impl Collector for NetworkCollector {
    fn register_metrics(&self) -> prometheus::Result<()> {
        Ok(())
    }

    fn collect_metrics(&self) {
        let mut networks = self.networks.lock().unwrap();
        networks.refresh();

        let mut total_received = 0u64;
        let mut total_transmitted = 0u64;
        let mut total_packets_received = 0u64;
        let mut total_packets_transmitted = 0u64;
        let mut total_errors_received = 0u64;
        let mut total_errors_transmitted = 0u64;

        // Aggregate data from all network interfaces
        for (_interface_name, data) in networks.iter() {
            total_received += data.received();
            total_transmitted += data.transmitted();
            total_packets_received += data.packets_received();
            total_packets_transmitted += data.packets_transmitted();
            total_errors_received += data.errors_on_received();
            total_errors_transmitted += data.errors_on_transmitted();
        }

        // Update gauge metrics (current values)
        self.network_bytes_received.set(total_received as f64);
        self.network_bytes_transmitted.set(total_transmitted as f64);
        self.network_packets_received
            .set(total_packets_received as f64);
        self.network_packets_transmitted
            .set(total_packets_transmitted as f64);
        self.network_errors_received
            .set(total_errors_received as f64);
        self.network_errors_transmitted
            .set(total_errors_transmitted as f64);

        // Update counter metrics (increment by current values)
        // Note: In a real implementation, you'd track the delta since last measurement
        self.network_bytes_received_total
            .inc_by(total_received as f64);
        self.network_bytes_transmitted_total
            .inc_by(total_transmitted as f64);
        self.network_packets_received_total
            .inc_by(total_packets_received as f64);
        self.network_packets_transmitted_total
            .inc_by(total_packets_transmitted as f64);

        // Simulate network latency for histogram (for learning purposes)
        // In real implementation, you'd measure actual network latency
        let simulated_latency = (total_received as f64 / 1000000.0).clamp(0.001, 10.0);
        self.network_latency_histogram.observe(simulated_latency);
    }
}
