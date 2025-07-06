mod collectors;
mod loader;

use crate::collectors::Collectors;
use crate::loader::Loader;
use serde_json::Value;
use std::env;
use std::path::Path;

fn main() {
    let folder = env::args().nth(1).expect("Missing folder path");

    let mut loader = Loader::new(Path::new(&folder));

    let collectors = Collectors::default();

    loader.process(|event| {
        collectors.collect(event);
    });

    let mut output = collectors.output();
    output.insert(String::from("file_count"), Value::from(loader.file_count()));
    println!("{}", Value::from(output));
}
