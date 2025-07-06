use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
pub struct GithubEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub created_at: Option<String>,
    // pub actor: Value,
    // pub repo: Value,
    // pub org: Option<Value>,
    pub payload: Value,
}

pub struct Loader {
    readers: Vec<(PathBuf, BufReader<File>)>,
}

impl Loader {
    pub fn new(folder: &Path) -> Self {
        let mut readers = Vec::new();

        for entry in fs::read_dir(folder).expect("Cannot read dir") {
            let path = entry.unwrap().path();
            if path.is_file() {
                readers.push((path.clone(), BufReader::new(File::open(&path).unwrap())));
            }
        }

        Self { readers }
    }

    pub fn process<F>(&mut self, f: F)
    where
        F: Fn(GithubEvent),
    {
        for (path, reader) in &mut self.readers {
            for line in reader.lines() {
                match serde_json::from_str::<GithubEvent>(&line.unwrap()) {
                    Ok(event) => f(event),
                    Err(e) => eprintln!(
                        "Failed to parse GithubEvent in file {}: {}",
                        path.display(),
                        e
                    ),
                }
            }
        }
    }

    pub fn file_count(&self) -> usize {
        self.readers.len()
    }
}
