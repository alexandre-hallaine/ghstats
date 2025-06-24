use std::collections::HashMap;
use serde_json::{json, Value};
use chrono::Timelike;
use crate::loaders::GithubEvent;
use crate::collectors::Collector;

/// Hourly activity collector - tracks activity patterns throughout the day (0-23 hours)
pub struct HourlyCollector;

impl Collector for HourlyCollector {
    fn name(&self) -> &str {
        "hourly_activity"
    }

    fn collect(&self, events: &[GithubEvent]) -> Value {
        let mut hourly_counts: HashMap<u8, usize> = HashMap::new();
        
        for event in events {
            if let Some(created_at) = &event.created_at {
                // Parse the ISO 8601 timestamp to extract hour
                if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(created_at) {
                    let hour = datetime.hour() as u8;
                    *hourly_counts.entry(hour).or_insert(0) += 1;
                }
            }
        }

        // Create entries for all 24 hours (0-23), even if no events
        let mut result: Vec<Value> = Vec::new();
        for hour in 0..24 {
            let count = hourly_counts.get(&hour).copied().unwrap_or(0);
            result.push(json!({"hour": hour, "count": count}));
        }

        json!(result)
    }
} 