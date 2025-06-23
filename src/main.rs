use std::{collections::HashMap, env, fs, io::BufRead, path::PathBuf};
use flate2::read::GzDecoder;
use rayon::prelude::*;
use serde::{Deserialize};
use serde_json::{json, Value};

#[derive(Deserialize)]
struct GithubEvent {
    #[serde(rename = "type")]
    event_type: String,
    payload: Value,
}

impl GithubEvent {
    fn get_language(&self) -> Option<String> {
        if self.event_type != "PullRequestEvent" {
            return None;
        }
        self.payload
            .pointer("/pull_request/base/repo/language")
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty() && *s != "null")
            .map(str::to_owned)
    }
}

fn main() {
    let dir = env::args().nth(1).expect("Usage: <dir>");
    let files: Vec<PathBuf> = fs::read_dir(&dir)
        .expect("Failed to read dir")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|p| p.extension().map_or(false, |ext| ext == "gz"))
        .collect();

    let total: HashMap<String, usize> = files
        .par_iter()
        .map(|path| {
            let file = match fs::File::open(path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Failed to open {}: {}", path.display(), e);
                    return HashMap::new();
                }
            };

            let reader = std::io::BufReader::new(GzDecoder::new(file));
            
            reader
                .lines()
                .filter_map(Result::ok)
                .filter_map(|line| serde_json::from_str::<GithubEvent>(&line).ok())
                .filter_map(|event| event.get_language())
                .fold(HashMap::new(), |mut acc, lang| {
                    *acc.entry(lang).or_insert(0) += 1;
                    acc
                })
        })
        .reduce(HashMap::new, |mut acc, map| {
            for (k, v) in map {
                *acc.entry(k).or_insert(0) += v;
            }
            acc
        });

    let mut langs: Vec<_> = total.into_iter().collect();
    langs.sort_unstable_by_key(|&(_, count)| std::cmp::Reverse(count));

    let languages: Vec<Value> = langs
        .into_iter()
        .map(|(lang, count)| json!({"language": lang, "count": count}))
        .collect();

    let output = json!({
        "languages": languages,
        "files_processed": files.len()
    });

    println!("{}", serde_json::to_string_pretty(&output).unwrap());
} 