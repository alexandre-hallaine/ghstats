mod loaders;
mod collectors;

use loaders::load_from_dir;
use collectors::Collectors;

fn main() {
    let dir = std::env::args().nth(1).expect("Usage: <dir>");
    let events = load_from_dir(&dir).expect("Failed to load events");
    let output = Collectors::default().collect_all(&events);
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}
