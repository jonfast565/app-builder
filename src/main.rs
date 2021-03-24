use std::fs;
use std::io::{Error};
use crate::appbuilder::AppBuilder;

mod dbbuilder;
mod utilities;
mod appbuilder;

fn main() -> Result<(), Error> {
    println!("--- App Builder ---");

    let filename = "./config.json";
    println!("Reading config...");
    let contents = fs::read_to_string(filename)?;
    let app_builder = AppBuilder::init(contents);
    app_builder.template();

    println!("Done!");
    Ok(())
}
