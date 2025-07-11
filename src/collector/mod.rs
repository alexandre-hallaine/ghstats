use crate::Event;
use serde_json::Value;

mod leaf;
mod master;

pub use master::MasterCollector;

pub trait Collector {
    fn name(&self) -> &str;
    fn handle(&mut self, event: &Event);
    fn finalize(&self) -> Value;
}
