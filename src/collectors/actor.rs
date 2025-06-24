use std::collections::HashMap;
use serde_json::{json, Value};
use crate::loaders::GithubEvent;
use crate::collectors::Collector;

/// Actor collector - counts most active users by event count (top 100)
pub struct ActorCollector;

impl Collector for ActorCollector {
    fn name(&self) -> &str {
        "top_actors"
    }

    fn collect(&self, events: &[GithubEvent]) -> Value {
        let mut total: HashMap<String, usize> = HashMap::new();
        
        for event in events {
            if let Some(actor) = event.actor.get("login").and_then(Value::as_str) {
                *total.entry(actor.to_string()).or_insert(0) += 1;
            }
        }

        let mut actors: Vec<_> = total.into_iter().collect();
        actors.sort_unstable_by_key(|&(_, count)| std::cmp::Reverse(count));

        // Take top 100 actors
        let result: Vec<Value> = actors
            .into_iter()
            .take(100)
            .map(|(actor, count)| json!({"actor": actor, "count": count}))
            .collect();

        json!(result)
    }
} 