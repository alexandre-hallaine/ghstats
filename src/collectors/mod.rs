use serde_json::Value;
use crate::loaders::GithubEvent;
use rayon::prelude::*;

pub mod language;
pub mod event_type;
pub mod repository;
pub mod actor;
pub mod hourly;
pub mod org;

pub use language::LanguageCollector;
pub use event_type::EventTypeCollector;
pub use repository::RepositoryCollector;
pub use actor::ActorCollector;
pub use hourly::HourlyCollector;
pub use org::OrgCollector;

pub trait Collector: Send + Sync {
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
        let results: Vec<(String, Value)> = self.collectors.par_iter()
            .map(|collector| {
                let name = collector.name().to_string();
                let data = collector.collect(events);
                (name, data)
            })
            .collect();
        Value::Object(results.into_iter().collect())
    }
}

impl Default for Collectors {
    fn default() -> Self {
        let mut manager = Self::new();
        manager.add_collector(Box::new(LanguageCollector));
        manager.add_collector(Box::new(EventTypeCollector));
        manager.add_collector(Box::new(RepositoryCollector));
        manager.add_collector(Box::new(ActorCollector));
        manager.add_collector(Box::new(HourlyCollector));
        manager.add_collector(Box::new(OrgCollector));
        manager
    }
}
