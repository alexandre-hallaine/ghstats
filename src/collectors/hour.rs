use crate::collectors::Collector;
use crate::loader::GithubEvent;
use chrono::Timelike;
use serde_json::Value;
use std::collections::HashMap;

pub struct HourCollector {
    amount: HashMap<u8, usize>,
}

impl HourCollector {
    pub fn new() -> Self {
        Self {
            amount: HashMap::new(),
        }
    }
}

impl Collector for HourCollector {
    fn collect(&mut self, event: &GithubEvent) {
        if let Some(created_at) = &event.created_at {
            let datetime = chrono::DateTime::parse_from_rfc3339(created_at).unwrap();
            let hour = datetime.hour() as u8;
            *self.amount.entry(hour).or_insert(0) += 1;
        }
    }

    fn output(&self) -> Value {
        serde_json::json!(self.amount)
    }
}
