use std::fs;

mod dbbuilder;
use dbbuilder::{DbSchema};

fn main() {
    println!("--- App Builder ---");

    let filename = "./config.json";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let deserialized: DbSchema = serde_json::from_str(&contents).unwrap();

    dbg!(deserialized);
}
