use std::collections::HashMap;
use serde_json::{json, Value};
use crate::loaders::GithubEvent;
use crate::collectors::Collector;

/// Repository collector - counts most active repositories by event count (top 100)
pub struct RepositoryCollector;

impl Collector for RepositoryCollector {
    fn name(&self) -> &str {
        "top_repositories"
    }

    fn collect(&self, events: &[GithubEvent]) -> Value {
        let mut total: HashMap<String, usize> = HashMap::new();
        
        for event in events {
            if let Some(repo) = event.repo.get("name").and_then(Value::as_str) {
                *total.entry(repo.to_string()).or_insert(0) += 1;
            }
        }

        let mut repositories: Vec<_> = total.into_iter().collect();
        repositories.sort_unstable_by_key(|&(_, count)| std::cmp::Reverse(count));

        // Take top 100 repositories
        let result: Vec<Value> = repositories
            .into_iter()
            .take(100)
            .map(|(repository, count)| json!({"repository": repository, "count": count}))
            .collect();

        json!(result)
    }
} 