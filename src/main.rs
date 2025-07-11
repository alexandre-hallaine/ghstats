use flate2::read::GzDecoder;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::{Deserializer, Value};
use std::env;
use std::io::BufReader;

mod collector;
use collector::{Collector, MasterCollector};

#[derive(Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    pub typ: String,
    pub created_at: String,
    pub actor: Value,
    pub repo: Value,
    pub org: Option<Value>,
    pub payload: Value,
}

fn main() {
    let date = env::args().nth(1).expect("Usage: ghstats YYYY-MM-DD");
    let client = Client::builder().build().unwrap();

    let mut collector = MasterCollector::default();

    for hour in 0..24 {
        let url = format!("https://data.gharchive.org/{}-{}.json.gz", date, hour);
        process(&client, &url, &mut collector);
    }

    println!("{}", collector.finalize());
}

fn process(client: &Client, url: &str, collector: &mut dyn Collector) {
    let data = client.get(url).send().unwrap();
    let decoder = GzDecoder::new(data);
    let reader = BufReader::new(decoder);
    let stream = Deserializer::from_reader(reader).into_iter::<Event>();

    for event in stream {
        collector.handle(&event.unwrap());
    }
}
