# MetrixD - Prometheus Metrics Guide

This guide explains the different types of Prometheus metrics implemented in MetrixD and their use cases.

## 📊 **Metric Types Overview**

MetrixD implements **3 main Prometheus metric types** to demonstrate different monitoring patterns:

### 1. **Gauge Metrics** 📈
**What**: Current snapshot values that can go up or down
**When to use**: For values that represent a current state
**Examples**: CPU usage, memory usage, disk space, temperature

```prometheus
# HELP cpu_usage_percent Current CPU usage percentage
# TYPE cpu_usage_percent gauge
cpu_usage_percent 1.81

# HELP memory_usage_percent Memory usage in percentage  
# TYPE memory_usage_percent gauge
memory_usage_percent 30.67
```

### 2. **Counter Metrics** 📊
**What**: Cumulative values that only increase (or reset to zero)
**When to use**: For counting events, bytes transferred, requests processed
**Examples**: Total network bytes, disk I/O operations, HTTP requests

```prometheus
# HELP network_bytes_received_total Total network bytes received since start
# TYPE network_bytes_received_total counter
network_bytes_received_total 537054

# HELP cpu_time_user_seconds_total Total CPU time spent in user mode
# TYPE cpu_time_user_seconds_total counter
cpu_time_user_seconds_total 13.04
```

### 3. **Histogram Metrics** 📊
**What**: Distribution of values across predefined buckets
**When to use**: For measuring latencies, request durations, sizes
**Examples**: Response times, disk operation latency, CPU load distribution

```prometheus
# HELP cpu_load_distribution Distribution of CPU load measurements
# TYPE cpu_load_distribution histogram
cpu_load_distribution_bucket{le="10"} 1
cpu_load_distribution_bucket{le="25"} 2
cpu_load_distribution_bucket{le="50"} 2
cpu_load_distribution_sum 22.22
cpu_load_distribution_count 2
```

## 🔧 **Implemented Metrics by Category**

### **CPU Metrics**
- `cpu_usage_percent` (Gauge) - Current CPU usage percentage
- `cpu_cores_total` (Gauge) - Total number of CPU cores
- `cpu_frequency_mhz` (Gauge) - Current CPU frequency in MHz
- `cpu_time_user_seconds_total` (Counter) - Total CPU time in user mode
- `cpu_time_system_seconds_total` (Counter) - Total CPU time in system mode
- `cpu_time_idle_seconds_total` (Counter) - Total CPU time idle
- `cpu_load_distribution` (Histogram) - Distribution of CPU load measurements

### **Memory Metrics**
- `memory_usage_percent` (Gauge) - Memory usage percentage
- `memory_total_bytes` (Gauge) - Total memory in bytes
- `memory_used_bytes` (Gauge) - Used memory in bytes
- `memory_available_bytes` (Gauge) - Available memory in bytes

### **Disk Metrics**
- `disk_usage_percent` (Gauge) - Disk usage percentage
- `disk_total_bytes` (Gauge) - Total disk space in bytes
- `disk_used_bytes` (Gauge) - Used disk space in bytes
- `disk_available_bytes` (Gauge) - Available disk space in bytes
- `disk_inodes_total` (Gauge) - Total number of inodes
- `disk_inodes_used` (Gauge) - Number of used inodes
- `disk_reads_total` (Counter) - Total disk read operations
- `disk_writes_total` (Counter) - Total disk write operations
- `disk_read_bytes_total` (Counter) - Total bytes read from disk
- `disk_write_bytes_total` (Counter) - Total bytes written to disk
- `disk_operation_duration_seconds` (Histogram) - Disk operation latency distribution

### **Network Metrics**
- `network_bytes_received` (Gauge) - Current network bytes received per second
- `network_bytes_transmitted` (Gauge) - Current network bytes transmitted per second
- `network_packets_received` (Gauge) - Current network packets received per second
- `network_packets_transmitted` (Gauge) - Current network packets transmitted per second
- `network_errors_received` (Gauge) - Current network errors received
- `network_errors_transmitted` (Gauge) - Current network errors transmitted
- `network_bytes_received_total` (Counter) - Total network bytes received
- `network_bytes_transmitted_total` (Counter) - Total network bytes transmitted
- `network_packets_received_total` (Counter) - Total network packets received
- `network_packets_transmitted_total` (Counter) - Total network packets transmitted
- `network_latency_seconds` (Histogram) - Network latency distribution

### **System Metrics**
- `load_average_1min` (Gauge) - System load average over 1 minute
- `load_average_5min` (Gauge) - System load average over 5 minutes
- `load_average_15min` (Gauge) - System load average over 15 minutes
- `uptime_seconds` (Gauge) - System uptime in seconds
- `process_count` (Gauge) - Number of running processes

## 🎯 **Learning Examples**

### **Gauge vs Counter**
```rust
// Gauge - can go up or down
let cpu_usage = register_gauge!("cpu_usage_percent", "Current CPU usage").unwrap();
cpu_usage.set(45.2); // Set to current value

// Counter - only increases
let requests_total = register_counter!("requests_total", "Total requests").unwrap();
requests_total.inc(); // Increment by 1
requests_total.inc_by(5.0); // Increment by 5
```

### **Histogram Buckets**
```rust
let latency_histogram = register_histogram!(
    "request_duration_seconds",
    "Request duration distribution",
    vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
).unwrap();

latency_histogram.observe(0.045); // Record a 45ms request
```

## 📈 **Prometheus Queries**

### **Rate Calculations**
```promql
# Network bytes received per second (5-minute rate)
rate(network_bytes_received_total[5m])

# CPU time rate
rate(cpu_time_user_seconds_total[5m])
```

### **Histogram Percentiles**
```promql
# 95th percentile of disk operation latency
histogram_quantile(0.95, disk_operation_duration_seconds_bucket)

# Average disk operation latency
rate(disk_operation_duration_seconds_sum[5m]) / rate(disk_operation_duration_seconds_count[5m])
```

### **Gauge Aggregations**
```promql
# Average CPU usage
avg(cpu_usage_percent)

# Memory usage ratio
memory_used_bytes / memory_total_bytes * 100
```

## 🚀 **Best Practices**

1. **Use Gauges** for current state values (temperature, memory usage)
2. **Use Counters** for cumulative values (requests, bytes transferred)
3. **Use Histograms** for latency and size distributions
4. **Choose appropriate bucket ranges** for histograms based on expected values
5. **Include units in metric names** (bytes, seconds, percent)
6. **Use consistent naming conventions** (snake_case, descriptive names)

## 🔍 **Monitoring Dashboard Ideas**

- **CPU Dashboard**: Usage trends, load distribution, core utilization
- **Memory Dashboard**: Usage over time, available memory alerts
- **Disk Dashboard**: Space usage, I/O rates, operation latencies
- **Network Dashboard**: Throughput, packet rates, error rates
- **System Dashboard**: Load averages, uptime, process count

This implementation demonstrates real-world monitoring patterns used in production systems!
