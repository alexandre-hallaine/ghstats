use std::collections::HashMap;
use serde_json::{json, Value};
use crate::loaders::GithubEvent;
use crate::collectors::Collector;

/// Organization collector - counts most active organizations by event count (top 100)
pub struct OrgCollector;

impl Collector for OrgCollector {
    fn name(&self) -> &str {
        "top_organizations"
    }

    fn collect(&self, events: &[GithubEvent]) -> Value {
        let mut total: HashMap<String, usize> = HashMap::new();
        
        for event in events {
            if let Some(org) = event.org.as_ref().and_then(|o| o.get("login")).and_then(Value::as_str) {
                *total.entry(org.to_string()).or_insert(0) += 1;
            }
        }

        let mut orgs: Vec<_> = total.into_iter().collect();
        orgs.sort_unstable_by_key(|&(_, count)| std::cmp::Reverse(count));

        // Take top 100 organizations
        let result: Vec<Value> = orgs
            .into_iter()
            .take(100)
            .map(|(org, count)| json!({"organization": org, "count": count}))
            .collect();

        json!(result)
    }
} 