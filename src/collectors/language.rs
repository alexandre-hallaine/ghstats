use std::collections::HashMap;
use serde_json::{json, Value};
use crate::loaders::GithubEvent;
use crate::collectors::Collector;

/// Language collector - counts programming languages from pull request events
pub struct LanguageCollector;

impl Collector for LanguageCollector {
    fn name(&self) -> &str {
        "languages"
    }

    fn collect(&self, events: &[GithubEvent]) -> Value {
        let mut total: HashMap<String, usize> = HashMap::new();
        
        for event in events {
            if event.event_type == "PullRequestEvent" {
                if let Some(lang) = event.payload
                    .pointer("/pull_request/base/repo/language")
                    .and_then(Value::as_str)
                    .filter(|s| !s.is_empty() && *s != "null")
                {
                    *total.entry(lang.to_string()).or_insert(0) += 1;
                }
            }
        }

        let mut langs: Vec<_> = total.into_iter().collect();
        langs.sort_unstable_by_key(|&(_, count)| std::cmp::Reverse(count));

        let languages: Vec<Value> = langs
            .into_iter()
            .map(|(lang, count)| json!({"language": lang, "count": count}))
            .collect();

        json!(languages)
    }
}
