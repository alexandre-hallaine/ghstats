use crate::Event;
use crate::collector::Collector;
use serde_json::Value;

#[derive(Default)]
pub struct CounterCollector {
    count: u64,
}

impl Collector for CounterCollector {
    fn name(&self) -> &str {
        "counter"
    }

    fn handle(&mut self, _: &Event) {
        self.count += 1;
    }

    fn finalize(&self) -> Value {
        serde_json::to_value(self.count).unwrap()
    }
}
