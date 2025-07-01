use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct GithubEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub created_at: Option<String>,
    pub actor: Value,
    pub repo: Value,
    pub org: Option<Value>,
    pub payload: Value,
}

pub struct Loader {
    readers: Vec<BufReader<File>>,
}

impl Loader {
    pub fn new(folder: &Path) -> Self {
        let mut readers = Vec::new();

        for entry in fs::read_dir(folder).expect("Cannot read dir") {
            let path = entry.unwrap().path();
            if path.is_file() {
                readers.push(BufReader::new(File::open(path).unwrap()));
            }
        }

        Self { readers }
    }

    pub fn process<F>(&mut self, f: F)
        where F: Fn(GithubEvent),
    {
        for reader in &mut self.readers {
            for line in reader.lines() {
                let event = serde_json::from_str::<GithubEvent>(&line.unwrap()).expect("Cannot parse GithubEvent");
                f(event);
            }
        }
    }
}
