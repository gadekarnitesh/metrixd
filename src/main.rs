use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::task;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use prometheus::{gather, Encoder, TextEncoder};
use std::net::SocketAddr;

mod collector;
mod metrics;

use crate::collector::Collector;
use crate::metrics::{
    CpuCollector, DiskCollector, MemoryCollector, NetworkCollector, SystemCollector,
};
#[tokio::main]
async fn main() {
    // Create your collectors
    let collectors: Vec<Box<dyn Collector + Send + Sync>> = vec![
        Box::new(CpuCollector::new()),
        Box::new(MemoryCollector::new()),
        Box::new(DiskCollector::new()),
        Box::new(SystemCollector::new()),
        Box::new(NetworkCollector::new()),
    ];

    // Register all metrics
    for collector in &collectors {
        collector
            .register_metrics()
            .expect("register_metrics failed");
    }

    // Wrap in Arc<Mutex> to share safely with async tasks
    let collectors = Arc::new(Mutex::new(collectors));

    // Spawn a background task to update metrics periodically
    {
        let collectors = Arc::clone(&collectors);
        task::spawn(async move {
            loop {
                {
                    let collectors = collectors.lock().await;
                    for collector in collectors.iter() {
                        collector.collect_metrics();
                        println!("Collected metrics..");
                    }
                }
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });
    }

    // Start HTTP server to expose metrics
    let addr = SocketAddr::from(([0, 0, 0, 0], 9100));

    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(metrics_handler)) });

    println!("Serving metrics on http://{}", addr);

    Server::bind(&addr).serve(make_svc).await.unwrap();
}

async fn metrics_handler(_req: Request<Body>) -> std::result::Result<Response<Body>, hyper::Error> {
    let encoder = TextEncoder::new();
    let metric_families = gather();
    let mut buffer = Vec::new();

    if let Err(e) = encoder.encode(&metric_families, &mut buffer) {
        eprintln!("Failed to encode metrics: {}", e);
        return Ok(Response::builder()
            .status(500)
            .body(Body::from("Internal Server Error"))
            .unwrap());
    }

    Ok(Response::builder()
        .header("Content-Type", encoder.format_type())
        .body(Body::from(buffer))
        .unwrap())
}
