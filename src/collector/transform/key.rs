use std::collections::HashMap;
use serde_json::{Value, Map};
use crate::collector::Collector;
use crate::Event;

pub struct PerKeyCollector
{
    map: HashMap<String, Box<dyn Collector>>,
    key_extractor: Box<dyn Fn(&Event) -> Option<String>>,
    factory: Box<dyn Fn() -> Box<dyn Collector>>,
}

impl PerKeyCollector {
    pub fn new(
        key_extractor: impl Fn(&Event) -> Option<String> + 'static,
        factory: impl Fn() -> Box<dyn Collector> + 'static,
    ) -> Self {
        Self {
            map: HashMap::new(),
            key_extractor: Box::new(key_extractor),
            factory: Box::new(factory),
        }
    }
}

impl Collector for PerKeyCollector {
    fn name(&self) -> &str {
        "per_key"
    }

    fn handle(&mut self, event: &Event) {
        if let Some(key) = (self.key_extractor)(event) {
            let collector = self.map.entry(key).or_insert_with(|| (self.factory)());
            collector.handle(event);
        }
    }

    fn finalize(&self) -> Value {
        let mut map = Map::new();

        for (key, collector) in &self.map {
            map.insert(key.clone(), collector.finalize());
        }

        Value::Object(map)
    }
}
