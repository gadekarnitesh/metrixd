# MetrixD - System Metrics Daemon

A lightweight, high-performance system metrics collector written in Rust that exposes Prometheus-compatible metrics via HTTP.

## Features

- **Real-time System Metrics**: CPU usage, memory utilization, disk space, and system load
- **Prometheus Compatible**: Metrics exposed in Prometheus format on `/metrics` endpoint
- **Low Resource Usage**: Written in Rust for minimal memory and CPU footprint
- **Docker Ready**: Multi-stage Docker build for easy deployment
- **Production Ready**: Includes health checks, proper error handling, and logging

## Metrics Collected

### CPU Metrics
- `cpu_usage_percent`: Current CPU usage percentage across all cores

### Memory Metrics
- `memory_usage_percent`: Memory usage as a percentage of total memory
- `memory_total_bytes`: Total system memory in bytes
- `memory_used_bytes`: Currently used memory in bytes
- `memory_available_bytes`: Available memory in bytes

### Disk Metrics
- `disk_usage_percent`: Disk usage percentage for root filesystem
- `disk_total_bytes`: Total disk space in bytes
- `disk_used_bytes`: Used disk space in bytes
- `disk_available_bytes`: Available disk space in bytes

### System Metrics
- `load_average_1min`: System load average over 1 minute
- `load_average_5min`: System load average over 5 minutes
- `load_average_15min`: System load average over 15 minutes
- `uptime_seconds`: System uptime in seconds
- `process_count`: Number of running processes

## Quick Start

### Using Docker (Recommended)

```bash
# Pull and run the latest image
docker run -d \
  --name metrixd \
  --restart unless-stopped \
  -p 9100:9100 \
  ghcr.io/yourusername/metrixd:latest

# Check metrics
curl http://localhost:9100/metrics
```

### Building from Source

#### Prerequisites
- Rust 1.75 or later
- Git

#### Build and Run
```bash
# Clone the repository
git clone https://github.com/yourusername/metrixd.git
cd metrixd

# Build in release mode
cargo build --release

# Run the daemon
./target/release/metrixd
```

The metrics server will start on `http://localhost:9100/metrics`

## Configuration

MetrixD currently uses sensible defaults:
- **Port**: 9100 (Prometheus node_exporter standard)
- **Metrics Collection Interval**: 5 seconds
- **Bind Address**: 0.0.0.0 (all interfaces)

## Docker Deployment

### Docker Compose

Create a `docker-compose.yml`:

```yaml
version: '3.8'
services:
  metrixd:
    image: ghcr.io/yourusername/metrixd:latest
    container_name: metrixd
    restart: unless-stopped
    ports:
      - "9100:9100"
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:9100/metrics"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
```

Run with:
```bash
docker-compose up -d
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: metrixd
spec:
  replicas: 1
  selector:
    matchLabels:
      app: metrixd
  template:
    metadata:
      labels:
        app: metrixd
    spec:
      containers:
      - name: metrixd
        image: ghcr.io/yourusername/metrixd:latest
        ports:
        - containerPort: 9100
        livenessProbe:
          httpGet:
            path: /metrics
            port: 9100
          initialDelaySeconds: 30
          periodSeconds: 10
---
apiVersion: v1
kind: Service
metadata:
  name: metrixd-service
spec:
  selector:
    app: metrixd
  ports:
  - protocol: TCP
    port: 9100
    targetPort: 9100
  type: ClusterIP
```

## Prometheus Integration

Add to your `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'metrixd'
    static_configs:
      - targets: ['localhost:9100']
    scrape_interval: 15s
    metrics_path: /metrics
```

## Development

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

### Local Development
```bash
# Run with debug logging
RUST_LOG=debug cargo run
```

## Architecture

MetrixD follows a modular collector pattern:
- **Collector Trait**: Defines the interface for metric collectors
- **Individual Collectors**: CPU, Memory, Disk, and System collectors
- **Async Runtime**: Uses Tokio for efficient async operations
- **HTTP Server**: Hyper-based server for metrics endpoint

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Monitoring and Alerting

### Grafana Dashboard

Import the provided Grafana dashboard (coming soon) or create custom panels using the exposed metrics.

### Sample Prometheus Alerts

```yaml
groups:
- name: metrixd.rules
  rules:
  - alert: HighCPUUsage
    expr: cpu_usage_percent > 80
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "High CPU usage detected"
      
  - alert: HighMemoryUsage
    expr: memory_usage_percent > 90
    for: 2m
    labels:
      severity: critical
    annotations:
      summary: "High memory usage detected"
```

## Performance

MetrixD is designed for minimal resource usage:
- **Memory**: ~5-10MB RSS
- **CPU**: <1% on modern systems
- **Disk I/O**: Minimal, only for metrics collection

## Troubleshooting

### Common Issues

1. **Permission Denied**: Ensure the binary has execute permissions
2. **Port Already in Use**: Check if another service is using port 9100
3. **Metrics Not Updating**: Verify system permissions for reading `/proc` filesystem

### Logs

Enable debug logging:
```bash
RUST_LOG=debug ./metrixd
```

## Roadmap

- [ ] Configuration file support
- [ ] Additional metrics (network I/O, process-specific metrics)
- [ ] Grafana dashboard templates
- [ ] Windows support
- [ ] Custom metric collection intervals per collector
