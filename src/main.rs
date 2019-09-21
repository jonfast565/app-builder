#[macro_use]
extern crate tera;
/*
#[macro_use]
extern crate serde_json;
*/

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
    let deserialized: DbSchema = serde_json::from_str(&contents).unwrap();
    // dbg!(&deserialized);
    let template = Templater{}.template(deserialized);
    // dbg!(&template);
    let mut output = File::create("./sql-result.sql")?;
    write!(output, "{}", template)
}
