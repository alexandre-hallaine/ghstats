mod collectors;
mod loader;

use crate::collectors::Collectors;
use crate::loader::Loader;
use std::env;
use std::path::Path;

fn main() {
    let folder = env::args().nth(1).expect("Missing folder path");

    let mut loader = Loader::new(Path::new(&folder));

    let collectors = Collectors::default();

    loader.process(|event| {
        collectors.collect(event);
    });

    println!("{}", collectors.output());
}
