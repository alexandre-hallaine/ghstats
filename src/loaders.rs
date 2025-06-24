use std::{fs, io::BufRead, path::PathBuf};
use flate2::read::GzDecoder;
use rayon::prelude::*;
use serde::Deserialize;
use serde_json::Value;

/// A single GitHub event, deserialized from archive files.
#[derive(Deserialize)]
pub struct GithubEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub payload: Value,
    pub actor: Value,
    pub repo: Value,
    pub org: Option<Value>,
    pub created_at: Option<String>,
}

pub fn load_from_dir(dir: &str) -> Result<Vec<GithubEvent>, Box<dyn std::error::Error>> {
    let files: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|p| p.extension().map_or(false, |ext| ext == "gz"))
        .collect();

    Ok(files.par_iter().flat_map(load_from_file).collect())
}

pub fn load_from_file(file_path: &PathBuf) -> Vec<GithubEvent> {
    let file = match fs::File::open(file_path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    let reader = std::io::BufReader::new(GzDecoder::new(file));

    reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| serde_json::from_str::<GithubEvent>(&line).ok())
        .collect()
}
