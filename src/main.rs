use std::fs;
use std::fs::File;
use std::io::{Error, Write};

mod dbbuilder;
mod utilities;
mod appbuilder;

fn main() -> Result<(), Error> {
    println!("--- App Builder ---");

    let filename = "./config.json";
    println!("Reading config...");
    let contents = fs::read_to_string(filename)?;

    println!("Done!");
    Ok(())
}
