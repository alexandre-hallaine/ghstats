use crate::collectors::Collector;
use crate::loader::GithubEvent;
use serde_json::Value;
use std::collections::HashMap;

pub struct LanguageCollector {
    amount: HashMap<String, usize>,
}

impl LanguageCollector {
    pub fn new() -> Self {
        Self {
            amount: HashMap::new(),
        }
    }
}

impl Collector for LanguageCollector {
    fn collect(&mut self, event: &GithubEvent) {
        if event.event_type == "PullRequestEvent" {
            if let Some(lang) = event
                .payload
                .pointer("/pull_request/base/repo/language")
                .and_then(Value::as_str)
            {
                *self.amount.entry(lang.to_string()).or_insert(0) += 1;
            }
        }
    }

    fn output(&self) -> Value {
        serde_json::json!(self.amount)
    }
}
