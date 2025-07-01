mod event;
mod hour;
mod language;

use crate::collectors::event::EventCollector;
use crate::collectors::hour::HourCollector;
use crate::collectors::language::LanguageCollector;
use crate::loader::GithubEvent;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::{Sender, channel};
use std::thread;

pub trait Collector: Send + 'static {
    fn collect(&mut self, event: &GithubEvent);
    fn output(&self) -> Value;
}

pub struct Collectors {
    collectors: HashMap<String, (Sender<Arc<GithubEvent>>, thread::JoinHandle<Value>)>,
}

impl Collectors {
    pub fn new() -> Self {
        Self {
            collectors: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String, mut collector: Box<dyn Collector>) {
        let (tx, rx) = channel::<Arc<GithubEvent>>();
        let handle = thread::spawn(move || {
            for event in rx {
                collector.collect(&event);
            }
            collector.output()
        });

        self.collectors.insert(name, (tx, handle));
    }

    pub fn collect(&self, event: GithubEvent) {
        let shared = Arc::new(event);
        for (_, (tx, _)) in &self.collectors {
            tx.send(shared.clone()).expect("Could not send event");
        }
    }

    pub fn output(self) -> Value {
        let mut map = Map::new();

        for (name, (tx, handle)) in self.collectors {
            drop(tx);

            let output = handle.join().expect("Thread panicked");
            map.insert(name, output);
        }

        Value::Object(map)
    }
}

impl Default for Collectors {
    fn default() -> Self {
        let mut manager = Self::new();
        manager.add(String::from("events"), Box::new(EventCollector::new()));
        manager.add(String::from("hours"), Box::new(HourCollector::new()));
        manager.add(
            String::from("languages"),
            Box::new(LanguageCollector::new()),
        );
        manager
    }
}
