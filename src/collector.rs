use prometheus::Result;

pub trait Collector {
    fn register_metrics(&self) -> Result<()>;
    fn collect_metrics(&self);
}
