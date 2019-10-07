#[macro_use]
extern crate tera;
/*
#[macro_use]
extern crate serde_json;
*/
// extern crate futures;
// extern crate futures_state_stream;
// extern crate tokio;
extern crate tiberius;

use std::fs;
use std::fs::File;
use std::io::{Write, Error};

mod dbbuilder;
mod dbtemplater;

use crate::dbbuilder::DbSchema;
use crate::dbtemplater::Templater;

fn main() -> Result<(), Error> {
    println!("--- App Builder ---");

    let filename = "./config.json";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("Read config");

    let deserialized: DbSchema = serde_json::from_str(&contents).unwrap();
    // dbg!(&deserialized);
    println!("Got database schema");

    let template = Templater::init().template(deserialized);
    // dbg!(&template);
    println!("Generated template");

    let mut output = File::create("./sql-result.sql")?;
    println!("Wrote result");
    println!("--- Done ---");
    write!(output, "{}", template)
}
