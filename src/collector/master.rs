use crate::Event;
use crate::collector::Collector;
use crate::collector::leaf::CounterCollector;
use serde_json::{Map, Value};

pub struct MasterCollector {
    children: Vec<Box<dyn Collector>>,
}

impl Default for MasterCollector {
    fn default() -> Self {
        Self {
            children: vec![Box::new(CounterCollector::default())],
        }
    }
}

impl Collector for MasterCollector {
    fn name(&self) -> &str {
        "master"
    }

    fn handle(&mut self, event: &Event) {
        for child in &mut self.children {
            child.handle(event);
        }
    }

    fn finalize(&self) -> Value {
        let mut map = Map::new();

        for child in &self.children {
            map.insert(child.name().to_string(), child.finalize());
        }

        Value::Object(map)
    }
}
