use serde_json::Value;
use crate::loaders::GithubEvent;

pub mod language;
pub use language::LanguageCollector;

pub trait Collector {
    fn name(&self) -> &str;
    fn collect(&self, events: &[GithubEvent]) -> Value;
}

pub struct Collectors {
    collectors: Vec<Box<dyn Collector>>,
}

impl Collectors {
    pub fn new() -> Self {
        Self {
            collectors: Vec::new(),
        }
    }

    pub fn add_collector(&mut self, collector: Box<dyn Collector>) {
        self.collectors.push(collector);
    }

    pub fn collect_all(&self, events: &[GithubEvent]) -> Value {
        let mut result = serde_json::Map::new();
        for collector in &self.collectors {
            let name = collector.name();
            let data = collector.collect(events);
            result.insert(name.to_string(), data);
        }
        Value::Object(result)
    }
}

impl Default for Collectors {
    fn default() -> Self {
        let mut manager = Self::new();
        manager.add_collector(Box::new(LanguageCollector));
        manager
    }
}
