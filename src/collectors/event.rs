use crate::collectors::Collector;
use crate::loader::GithubEvent;
use serde_json::Value;
use std::collections::HashMap;

pub struct EventCollector {
    amount: HashMap<String, usize>,
}

impl EventCollector {
    pub fn new() -> Self {
        Self {
            amount: HashMap::new(),
        }
    }
}

impl Collector for EventCollector {
    fn collect(&mut self, event: &GithubEvent) {
        *self.amount.entry(event.event_type.clone()).or_insert(0) += 1;
    }

    fn output(&self) -> Value {
        serde_json::json!(self.amount)
    }
}
