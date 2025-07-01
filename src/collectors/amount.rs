use crate::collectors::Collector;
use crate::loader::GithubEvent;
use serde_json::{Value, json};

pub struct AmountCollector {
    count: usize,
}

impl AmountCollector {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Collector for AmountCollector {
    fn collect(&mut self, _: &GithubEvent) {
        self.count += 1;

        if self.count % 10000 == 0 {
            println!("{}", self.count);
        }
    }

    fn output(&self) -> Value {
        json!({
            "event_count": self.count
        })
    }
}
