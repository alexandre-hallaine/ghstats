mod loader;

use std::env;
use std::path::Path;
use crate::loader::Loader;

fn main() {
    let folder = env::args().nth(1).expect("Missing folder path");

    let mut loader = Loader::new(Path::new(&folder));
    loader.process(|event| {
        println!("{:?}", event);
    });
}
