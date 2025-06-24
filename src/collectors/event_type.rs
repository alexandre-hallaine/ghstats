use std::collections::HashMap;
use serde_json::{json, Value};
use crate::loaders::GithubEvent;
use crate::collectors::Collector;

/// Event type collector - counts different GitHub event types
pub struct EventTypeCollector;

impl Collector for EventTypeCollector {
    fn name(&self) -> &str {
        "event_types"
    }

    fn collect(&self, events: &[GithubEvent]) -> Value {
        let mut total: HashMap<String, usize> = HashMap::new();
        
        for event in events {
            *total.entry(event.event_type.clone()).or_insert(0) += 1;
        }

        let mut event_types: Vec<_> = total.into_iter().collect();
        event_types.sort_unstable_by_key(|&(_, count)| std::cmp::Reverse(count));

        let result: Vec<Value> = event_types
            .into_iter()
            .map(|(event_type, count)| json!({"event_type": event_type, "count": count}))
            .collect();

        json!(result)
    }
} 