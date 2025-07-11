use crate::Event;
use serde::Serialize;
use serde_json::Value;

pub trait Collector {
    fn collect(&mut self, event: &Event);
    fn finalize(&self) -> Value;
}

#[derive(Default, Serialize)]
pub struct CountPushEvents {
    count: u64,
}

impl Collector for CountPushEvents {
    fn collect(&mut self, event: &Event) {
        if event.typ == "PushEvent" {
            self.count += 1;
        }
    }

    fn finalize(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}
